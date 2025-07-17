use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ClientId(pub usize);

impl Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClientId({})", self.0)
    }
}
