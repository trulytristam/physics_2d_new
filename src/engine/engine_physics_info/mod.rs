#[derive(Clone)]
pub struct EnginePhysicsInfo {
    _gravity: f64,
}

impl Default for EnginePhysicsInfo {
    fn default() -> Self {
        EnginePhysicsInfo { _gravity: 9.18 }
    }
}
