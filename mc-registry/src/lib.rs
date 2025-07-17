use std::{
    fmt::Debug,
    ops::Deref,
    sync::{Arc, RwLock},
};

use pluggie::{
    AllLoadedEvent, event_ref::EventRef, exposable::Exposable, pluggie_context::PluggieCtx,
};

use crate::registry::Registry;

pub mod biome;
pub mod cat_variant;
pub mod chicken_variant;
pub mod cow_variant;
pub mod damage_type;
pub mod dimension_type;
pub mod frog_variant;
pub mod painting_variant;
pub mod pig_variant;
pub mod registry;
pub mod wolf_sound_variant;
pub mod wolf_variant;

pub extern "C" fn __pluggie_init(ctx: PluggieCtx) {
    init(ctx);
}

#[unsafe(no_mangle)]
#[cfg(feature = "init")]
pub extern "C" fn pluggie_def() -> pluggie::plugin::PluginRef {
    pluggie::plugin::PluginRef {
        init: __pluggie_init,
        plugin_info: pluggie::plugin::PluginInfo {
            name: "core:mc-registry".into(),
            version: "0.1.0".into(),
            author: "github.com:lukas0008".into(),
            pluggie_version: pluggie::VERSION,
        },
        load_early: true,
    }
}

#[derive(Clone)]
pub struct SharedRegistry(pub Arc<RwLock<Registry>>);
impl Deref for SharedRegistry {
    type Target = RwLock<Registry>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Debug for SharedRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SharedRegistry")
    }
}

impl Exposable for SharedRegistry {
    const NAME: &'static str = "core:mc-registry:SharedRegistry";
}

// this plugin should only expose a shared registry, others should populate it
fn init(ctx: PluggieCtx) {
    let shared = SharedRegistry(Arc::new(RwLock::new(Registry::new())));
    ctx.expose(shared);

    ctx.subscribe(|ev: EventRef<AllLoadedEvent>| {
        ev.ctx.info("All loaded");
    });
}
