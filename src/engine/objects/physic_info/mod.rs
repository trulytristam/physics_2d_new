use nalgebra;

type V2 = nalgebra::Vector2<f64>;

#[derive(Clone)]
pub struct PhysicInfo {
    pub pos: V2,
    pub vel: V2,
    pub ang: f64,
    pub imass: f64,
}

impl Default for PhysicInfo {
    fn default() -> Self {
        PhysicInfo {
            pos: V2::new(0., 0.),
            vel: V2::new(0., 0.),
            ang: 0.,
            imass: 0.1,
        }
    }
}

impl PhysicInfo {
    pub fn apply_impulse(&mut self, point: V2, force: V2) {
        let r_vec = point - self.pos;

        let x = r_vec.normalize().dot(&force.normalize());
        let f = force.magnitude();
        let r = r_vec.magnitude();
        self.ang = self.ang + f * r * x;
        self.vel = self.vel + force * (1. - x) * self.imass;
    }
}
