#![feature(duration_millis_float)]

use std::ops::Deref;

use mc_network::{events::NewConnectionEvent, network_context::NetworkContext};
use mc_tick::TickEvent;
use pluggie::{
    AllLoadedEvent, describe_plugin, event_ref::EventRef, pluggie_context::PluggieCtx,
    plugin::PluginInfo,
};

describe_plugin!(
    init,
    PluginInfo {
        name: "core:mc-example".into(),
        version: "0.1.0".into(),
        author: "github.com:lukas0008".into(),
        pluggie_version: pluggie::VERSION,
    }
);

fn init(ctx: PluggieCtx) {
    let start = std::time::Instant::now();
    ctx.subscribe(move |ev: EventRef<TickEvent>| {
        let elapsed = start.elapsed();
        if (ev.tick + 1) % 50 == 0 {
            ev.ctx.info(&format!(
                "Tps: {:.4}",
                ev.tick as f64 / elapsed.as_millis_f64() * 1000.
            ));
        }
    });
    ctx.subscribe(move |ev: EventRef<AllLoadedEvent>| {
        ev.ctx.info("All loaded");

        let net_ctx: NetworkContext = ev.ctx.get().unwrap();

        ev.ctx.subscribe(move |ev: EventRef<NewConnectionEvent>| {
            ev.ctx.info("conn in example");
            net_ctx.send_raw_packet(ev.deref().0, b"yogurt\0".to_vec());
        });
    });
}
