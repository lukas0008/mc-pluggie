#![feature(duration_millis_float)]
mod client;
mod define_network_context;
pub mod network_context;
pub mod network_loop;

use std::sync::{Arc, Mutex};

use dashmap::DashMap;
use mclib_network::{ClientMode, NewConnectionEvent, RawPacketEvent, ServerPacketEvent};
use mclib_protocol::{
    Packet, SPacket,
    packet_parsing::get_unparsed_packet_uncompressed,
    serde::deserializer::DeserializePacket,
    server::{
        config::{SConfigFinishAcknowledged, SConfigPacket, SKnownPacks},
        handshake::SHandshakePacket,
        login::{SLoginAcknowledged, SLoginPacket, SLoginStart},
        play::{SPlayPacket, SSetPlayerPos, SSetPlayerPosRot, STickEnd},
        status::{SPingRequest, SStatusPacket, SStatusRequest},
    },
};
use mio::Waker;
use pluggie::{
    AllLoadedEvent, curry::curry_once, describe_plugin, event_ref::EventRef,
    pluggie_context::PluggieCtx, plugin::PluginInfo,
};

use crate::{
    client::Client,
    define_network_context::defined_network_context,
    network_context::{NetworkContextImplementation, NetworkContextInternal},
    network_loop::network_loop,
};

describe_plugin!(
    init,
    PluginInfo {
        name: "core:mc-network".into(),
        version: "0.1.0".into(),
        author: "github.com:lukas0008".into(),
        pluggie_version: pluggie::VERSION,
    }
);

const WAKE_TOKEN: mio::Token = mio::Token(0);
const SERVER_TOKEN: mio::Token = mio::Token(1);

fn init(ctx: PluggieCtx) {
    // let connections = Arc::new(Mutex::new(HashMap::new()));
    let poll = mio::Poll::new().unwrap();
    let waker = Arc::new(Waker::new(poll.registry(), WAKE_TOKEN).unwrap());
    let (task_sender, task_receiver) = abi_stable::external_types::crossbeam_channel::unbounded();
    // let (task_sender, task_receiver) = std::sync::mpsc::sync_channel::<NetworkTask>(100);

    ctx.subscribe(|ev: EventRef<AllLoadedEvent>| {
        ev.ctx.info("All loaded");
    });
    let connections: Arc<DashMap<mio::Token, Client>> = Arc::new(DashMap::new());
    let net_ctx_impl = NetworkContextImplementation(Arc::new(Mutex::new(NetworkContextInternal {
        task_sender,
        waker: waker.clone(),
        connections: connections.clone(),
    })));
    let net_ctx = defined_network_context(net_ctx_impl);
    let new_connection_sender = ctx.register_event::<NewConnectionEvent>();
    let raw_packet_sender = ctx.register_event::<RawPacketEvent>();
    let server_packet_sender = ctx.register_event::<ServerPacketEvent>();
    ctx.subscribe(move |ev: EventRef<RawPacketEvent>| {
        let (packet_type, payload) = get_unparsed_packet_uncompressed(&ev.data).unwrap();

        macro_rules! match_packets {
            ($mode: ident, $( $packet: ident),*) => { paste::paste! {

                match packet_type {
                    $(
                        $packet::PACKET_ID => Some(
                            mclib_protocol::SPacket::$mode( [<S $mode Packet>] :: $packet ( [<$packet>] ::deserialize_packet(&payload).unwrap())
                        )),
                    )*
                    _ => None,
                }
                }
            }
        }

        // dbg!(packet_type);
        let packet = match ev.client_mode {
            ClientMode::Handshake => if SHandshakePacket::PACKET_ID == packet_type {
                Some(SPacket::Handshake(SHandshakePacket::deserialize_packet(&payload).unwrap()))
            } else {
                None
            },
            ClientMode::Status => match_packets!(Status, SPingRequest, SStatusRequest),
            ClientMode::Login => match_packets!(Login, SLoginStart, SLoginAcknowledged),
            ClientMode::Config => match_packets!(Config, SKnownPacks, SConfigFinishAcknowledged),
            ClientMode::Play => match_packets!(Play, SSetPlayerPos, SSetPlayerPosRot, STickEnd),
        };

        if let Some(packet) = packet {
            server_packet_sender.call(&ServerPacketEvent { client_id: ev.client_id, packet: packet });
        } else {
            ev.ctx.error(&format!("packet dont work: {:?} {:#x}", ev.client_mode, packet_type));
        }
    });

    ctx.expose(net_ctx.clone());
    std::thread::spawn(curry_once(network_loop)((
        ctx,
        poll,
        task_receiver,
        new_connection_sender,
        raw_packet_sender,
        connections,
    )));
}
