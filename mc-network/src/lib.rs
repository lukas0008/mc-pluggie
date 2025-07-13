#![feature(duration_millis_float)]
mod client;
pub mod client_id;
pub mod client_mode;
pub mod events;
pub mod network_context;
pub mod network_loop;

use std::sync::{Arc, Mutex};

use mclib_protocol::{
    Packet, SPacket,
    packet_parsing::get_unparsed_packet_uncompressed,
    serde::deserializer::DeserializePacket,
    server::{
        handshake::SHandshakePacket,
        status::{SPingRequest, SStatusPacket},
    },
};
use mio::Waker;
use pluggie::{
    AllLoadedEvent, curry::curry_once, describe_plugin, event_ref::EventRef,
    pluggie_context::PluggieCtx, plugin::PluginInfo,
};

use crate::{
    client_id::ClientId,
    client_mode::ClientMode,
    events::{NewConnectionEvent, RawPacketEvent},
    network_context::{NetworkContext, NetworkContextInternal},
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
    let net_ctx = NetworkContext(Arc::new(Mutex::new(NetworkContextInternal {
        task_sender,
        waker: waker.clone(),
        yo: 69,
    })));
    let new_connection_sender = ctx.register_event::<NewConnectionEvent>();
    let raw_packet_sender = ctx.register_event::<RawPacketEvent>();
    ctx.subscribe(|ev: EventRef<RawPacketEvent>| {
        ev.ctx.info("Raw packet received");
        ev.ctx.info(&format!("data: {:?}", &ev.data));

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

        let packet = match ev.client_mode {
            ClientMode::Handshake => if SHandshakePacket::PACKET_ID == packet_type {
                Some(SPacket::Handshake(SHandshakePacket::deserialize_packet(&payload).unwrap()))
            } else {
                None
            },
            ClientMode::Status => match_packets!(Status, SPingRequest),
            _ => None,
        };
        dbg!(packet);
    });
    // net_ctx.send_raw_packet(client_id, packet);
    ctx.expose(net_ctx.clone());
    std::thread::spawn(curry_once(network_loop)((
        ctx,
        poll,
        task_receiver,
        new_connection_sender,
        raw_packet_sender,
    )));
}
