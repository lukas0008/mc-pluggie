#![feature(duration_millis_float)]
pub mod client_id;
pub mod network_context;
pub mod network_loop;

use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use mio::Waker;
use pluggie::{
    AllLoadedEvent, curry::curry_once, describe_plugin, event::Event, event_ref::EventRef,
    pluggie_context::PluggieCtx, plugin::PluginInfo,
};

use crate::{
    client_id::ClientId,
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

#[derive(Debug)]
pub(crate) enum NetworkTask {
    SendPacket(ClientId, Vec<u8>),
}

#[derive(Debug)]
struct Client {
    conn: mio::net::TcpStream,
    #[expect(unused)]
    addr: SocketAddr,
    currently_writable: bool,
    to_write: Vec<u8>,
}

pub struct NewConnectionEvent(pub ClientId);
impl Event for NewConnectionEvent {
    const NAME: &'static str = "core:mc-network:new-connection";
}

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
    let event_sender = ctx.register_event::<NewConnectionEvent>();
    // net_ctx.send_raw_packet(client_id, packet);
    ctx.expose(net_ctx.clone());
    std::thread::spawn(curry_once(network_loop)((
        ctx,
        poll,
        task_receiver,
        event_sender,
    )));
}
