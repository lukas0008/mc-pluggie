use mc_proc::packet;

#[packet(id = 0x00)]
pub struct CStatusResponse {
    pub json_response: String,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Clone, Debug)]
pub struct StatusResponseJson {
    pub version: StatusResponseVersion,
    pub players: StatusResponsePlayers,
    pub description: StatusResponseDescription,
    pub favicon: Option<String>,
    pub enforces_secure_chat: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct StatusResponseVersion {
    pub name: String,
    pub protocol: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct StatusResponsePlayers {
    pub max: u32,
    pub online: u32,
    pub sample: Vec<StatusResponsePlayer>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct StatusResponsePlayer {
    pub name: String,
    pub id: String,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct StatusResponseDescription {
    pub text: String,
}
