#![feature(thread_sleep_until)]
use std::sync::Arc;

use abi_stable::StableAbi;
use pluggie::{
    AllLoadedEvent, describe_plugin, event::Event, event_ref::EventRef,
    pluggie_context::PluggieCtx, plugin::PluginInfo,
};

#[derive(StableAbi, Clone)]
#[repr(C)]
pub struct TickEvent {
    pub tick: u64,
}

impl Event for TickEvent {
    const NAME: &'static str = "core:mc-tick:tick";
}

describe_plugin!(
    init,
    PluginInfo {
        name: "core:mc-tick".into(),
        version: "0.1.0".into(),
        author: "github.com:lukas0008".into(),
        pluggie_version: pluggie::VERSION,
    }
);

fn init(ctx: PluggieCtx) {
    ctx.subscribe(move |ev: EventRef<AllLoadedEvent>| {
        ev.ctx.info("All loaded");
        let tick_ev = Arc::new(ev.ctx.register_event::<TickEvent>());
        let tick_ev = tick_ev.clone();

        let ctx = ev.ctx.clone();
        ev.post_event_hook(move |_| {
            ctx.info("Starting tick loop!");
            let tps = 20.;
            let mut tick = 0u64;
            let beginning = std::time::Instant::now();
            loop {
                tick_ev.call(&TickEvent { tick });
                tick += 1;
                spin_sleep::sleep_until(
                    beginning + std::time::Duration::from_secs_f64(tick as f64 / tps),
                );
            }
        });
    });
}
