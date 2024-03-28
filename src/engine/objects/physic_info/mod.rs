use nalgebra::{self};

type V2 = nalgebra::Vector2<f64>;

#[derive(Clone)]
pub struct PhysicInfo {
    pub pos: V2,
    pub vel: V2,
    pub ang: f64,
    pub w: f64,
    pub i_m: f64,
    pub i_i: f64,
}

impl Default for PhysicInfo {
    fn default() -> Self {
        PhysicInfo {
            pos: V2::new(0., 0.),
            vel: V2::new(0., 0.),
            ang: 0.,
            w: 0.,
            i_m: 1.,
            i_i: 1. / 5000.,
        }
    }
}

impl PhysicInfo {
    pub fn apply_impulse(&mut self, point_world: V2, force: V2) {
        let r = point_world - self.pos;
        let mut dir = 1.;
        if r.dot(&force) < 0. {
            dir = -1.;
        }
        self.w = self.w + force.perp(&r) * self.i_i;
        self.vel = self.vel - force.normalize() * (force.dot(&r.normalize()) * self.i_m) * dir;
    }
    pub fn kill_momentum(&mut self) {
        self.vel = V2::new(0., 0.);
        self.w = 0.;
    }
}
