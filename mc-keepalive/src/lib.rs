use std::sync::{Arc, Mutex};

use mc_tick::TickEvent;
use mclib_network::{NetworkContext, NewConnectionEvent};
use mclib_protocol::client::play::CKeepAlive;
use pluggie::{
    AllLoadedEvent, describe_plugin, event_ref::EventRef, pluggie_context::PluggieCtx,
    plugin::PluginInfo,
};

describe_plugin!(
    init,
    PluginInfo {
        name: "core:mc-keepalive".into(),
        version: "0.1.0".into(),
        author: "github.com:lukas0008".into(),
        pluggie_version: pluggie::VERSION,
    }
);

fn init(ctx: PluggieCtx) {
    ctx.subscribe(|ev: EventRef<AllLoadedEvent>| {
        ev.ctx.info("All loaded");
    });

    let connections = Arc::new(Mutex::new(Vec::new()));
    {
        let connections = connections.clone();
        ctx.subscribe(move |ev: EventRef<NewConnectionEvent>| {
            connections.lock().unwrap().push(ev.0);
        });
    }
    ctx.subscribe(move |ev: EventRef<TickEvent>| {
        // every 5 sec
        if ev.tick % (20 * 5) == 0 {
            let net_ctx = ev.ctx.get::<NetworkContext>().unwrap();
            let lock = connections.lock().unwrap();
            for conn in lock.iter() {
                net_ctx.send_packet(*conn, &CKeepAlive { id: 0 });
            }
        }
    });
}
