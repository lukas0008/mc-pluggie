use pluggie::{describe_plugin, pluggie_context::PluggieCtx, plugin::PluginInfo};

describe_plugin!(
    init,
    PluginInfo {
        name: "core:mc-loader".into(),
        version: "0.1.0".into(),
        author: "github.com:lukas0008".into(),
        pluggie_version: pluggie::VERSION.into(),
    }
);

fn init(ctx: PluggieCtx) {
    
}
