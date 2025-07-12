#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ClientId(pub usize);

impl ClientId {
    pub(crate) fn as_token(&self) -> mio::Token {
        mio::Token(self.0)
    }
}
