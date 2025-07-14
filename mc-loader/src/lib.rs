use mc_network::{
    client_mode::ClientMode, events::ServerPacketEvent, network_context::NetworkContext,
};
use mclib_protocol::{
    SPacket,
    client::{
        config::{CFinishConfig, CKnownPacks},
        login::CLoginSuccess,
        play::{CGameEvent, CSynchronizePlayerPosition},
        status::{
            CPingResponse, CStatusResponse,
            status_response::{
                StatusResponseDescription, StatusResponseJson, StatusResponsePlayers,
                StatusResponseVersion,
            },
        },
    },
    server::{config::SConfigPacket, login::SLoginPacket, status::SStatusPacket},
};
use pluggie::{
    AllLoadedEvent, describe_plugin, event_ref::EventRef, pluggie_context::PluggieCtx,
    plugin::PluginInfo,
};
use uuid::Uuid;

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
    ctx.subscribe(|ev: EventRef<AllLoadedEvent>| {
        ev.ctx.info("All loaded");
    });
    ctx.subscribe(|ev: EventRef<ServerPacketEvent>| {
        let net_ctx = ev
            .ctx
            .get::<NetworkContext>()
            .expect("NetworkContext not found");
        match &ev.packet {
            SPacket::Handshake(handshake) => match handshake.intent.0 {
                1 => {
                    ev.ctx.info("Switching to status mode");
                    net_ctx.switch_client_mode(ev.client_id, ClientMode::Status);
                }
                2 => {
                    net_ctx.switch_client_mode(ev.client_id, ClientMode::Login);
                }
                3 => {
                    todo!("transfers not supported, should kick")
                }
                _ => {
                    todo!("should kick")
                }
            },
            SPacket::Login(login_packet) => match &login_packet {
                SLoginPacket::SLoginStart(login_start_packet) => {
                    ev.ctx.info(&format!(
                        "Login started by {}",
                        &login_start_packet.username
                    ));

                    net_ctx.send_packet(
                        ev.client_id,
                        &CLoginSuccess {
                            uuid: Uuid::new_v4(),
                            username: login_start_packet.username.clone(),
                            properties: Vec::new().into(),
                        },
                    );
                }
                SLoginPacket::SLoginAcknowledged(_) => {
                    net_ctx.switch_client_mode(ev.client_id, ClientMode::Config);
                    net_ctx.send_packet(
                        ev.client_id,
                        &CKnownPacks {
                            known_packs: vec![(
                                "minecraft".to_string(),
                                "core".to_string(),
                                "1.21.7".to_string(),
                            )]
                            .into(),
                        },
                    );
                    net_ctx.send_packet(ev.client_id, &CFinishConfig {});
                }
                _ => {}
            },
            SPacket::Config(config_packet) => match config_packet {
                SConfigPacket::SConfigFinishAcknowledged(_) => {
                    net_ctx.switch_client_mode(ev.client_id, ClientMode::Play);
                    net_ctx.send_packet(
                        ev.client_id,
                        &CGameEvent {
                            event: 13,
                            value: 0.,
                        },
                    );
                    net_ctx.send_packet(
                        ev.client_id,
                        &CSynchronizePlayerPosition {
                            tp_id: 0.into(),
                            x: 0.0,
                            y: 5000.0,
                            z: 0.0,
                            vel_x: 0.0,
                            vel_y: 0.0,
                            vel_z: 0.0,
                            yaw: 0.0,
                            pitch: 0.0,
                            flags: 0,
                        },
                    );
                }
                _ => {}
            },
            _ => {}
        }
    });
}
