use mclib_core::PlayerId;

pub struct PlayerFuncs {
    pub client_id: Box<dyn Fn() -> PlayerId>,
}
