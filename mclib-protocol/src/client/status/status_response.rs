use mc_proc::packet;

#[packet(id = 0x00)]
pub struct CStatusResponse {
    pub json_response: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponseJson {
    pub version: StatusResponseVersion,
    pub players: StatusResponsePlayers,
    pub description: StatusResponseDescription,
    pub favicon: Option<String>,
    pub enforces_secure_chat: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct StatusResponseVersion {
    pub name: String,
    pub protocol: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct StatusResponsePlayers {
    pub max: u32,
    pub online: u32,
    pub sample: Vec<StatusResponsePlayer>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct StatusResponsePlayer {
    pub name: String,
    pub id: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct StatusResponseDescription {
    pub text: String,
}
