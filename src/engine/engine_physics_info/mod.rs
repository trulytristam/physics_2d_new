#[derive(Clone)]
pub struct EnginePhysicsInfo {
    gravity: f64,
}

impl Default for EnginePhysicsInfo {
    fn default() -> Self {
        EnginePhysicsInfo { gravity: 9.18 }
    }
}

