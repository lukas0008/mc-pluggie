#[derive(Clone, Copy, Debug)]
pub struct ClientId(pub usize);

impl ClientId {
    pub(crate) fn as_token(&self) -> mio::Token {
        mio::Token(self.0)
    }
}
