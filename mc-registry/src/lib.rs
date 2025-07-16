use std::{
    fmt::Debug,
    ops::Deref,
    sync::{Arc, Mutex, RwLock, RwLockReadGuard},
};

use arc_swap::ArcSwap;
use pluggie::{
    describe_plugin, exposable::Exposable, pluggie_context::PluggieCtx, plugin::PluginInfo,
};

use crate::registry::Registry;

pub mod dimension_type;
pub mod registry;

describe_plugin!(
    init,
    PluginInfo {
        name: "core:mc-registry".into(),
        version: "0.1.0".into(),
        author: "github.com:lukas0008".into(),
        pluggie_version: pluggie::VERSION,
    }
);

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
}
