use super::physics::collisions::EngineCollisionInfo;

#[derive(Clone)]
pub struct EnginePhysicsInfo {
    pub _gravity: f64,
    pub _collisions: EngineCollisionInfo,
}

impl Default for EnginePhysicsInfo {
    fn default() -> Self {
        EnginePhysicsInfo {
            _gravity: 9.18,
            _collisions: EngineCollisionInfo::default(),
        }
    }
}
