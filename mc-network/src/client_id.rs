use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ClientId(pub usize);

impl Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClientId({})", self.0)
    }
}

impl ClientId {
    pub(crate) fn as_token(&self) -> mio::Token {
        mio::Token(self.0)
    }
}
