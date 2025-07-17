#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientMode {
    Handshake,
    Status,
    Config,
    Login,
    Play,
}
