use mc_network::{events::ServerPacketEvent, network_context::NetworkContext};
use mclib_protocol::{
    SPacket,
    client::status::{
        CPingResponse, CStatusResponse,
        status_response::{
            StatusResponseDescription, StatusResponseJson, StatusResponsePlayers,
            StatusResponseVersion,
        },
    },
    server::status::SStatusPacket,
};
use pluggie::{
    AllLoadedEvent, describe_plugin, event_ref::EventRef, pluggie_context::PluggieCtx,
    plugin::PluginInfo,
};

describe_plugin!(
    init,
    PluginInfo {
        name: "core:mc-status".into(),
        version: "0.1.0".into(),
        author: "github.com:lukas0008".into(),
        pluggie_version: pluggie::VERSION.into(),
    }
);

fn init(ctx: PluggieCtx) {
    ctx.subscribe(|ev: EventRef<AllLoadedEvent>| {
        ev.ctx.info("All loaded");
    });
    ctx.subscribe(|ev: EventRef<ServerPacketEvent>| {
        let net_ctx = ev
            .ctx
            .get::<NetworkContext>()
            .expect("NetworkContext not found");
        match &ev.packet {
            SPacket::Status(status_packet) => match status_packet {
                SStatusPacket::SStatusRequest(_) => {
                    // dbg!("sending stuff");
                    let status = StatusResponseJson {
                        players: StatusResponsePlayers {
                            online: 0,
                            max: 100,
                            sample: Vec::new(),
                        },
                        version: StatusResponseVersion {
                            name: "1.21.5".into(),
                            protocol: 772,
                        },
                        description: StatusResponseDescription {
                            text: "I FUCKING HATE YOGURT".to_string(),
                        },
                        favicon: None,
                        enforces_secure_chat: false,
                    };
                    let status_str = serde_json::to_string(&status).unwrap();
                    // dbg!(&status_str);
                    net_ctx.send_packet(
                        ev.client_id,
                        &CStatusResponse {
                            json_response: status_str,
                        },
                    );
                    // net_ctx.close_client(ev.client_id);
                }
                SStatusPacket::SPingRequest(ping_request) => {
                    net_ctx.send_packet(
                        ev.client_id,
                        &CPingResponse {
                            payload: ping_request.payload,
                        },
                    );
                }
            },
            _ => {}
        }
    });
}
