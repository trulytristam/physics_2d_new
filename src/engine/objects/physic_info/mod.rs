use nalgebra;

use crate::engine::helper_functions::get_perp_matrix;

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
            i_m: 1. / 5.,
            i_i: 1. / 140.,
        }
    }
}

impl PhysicInfo {
    pub fn apply_impulse(&mut self, point_world: V2, force: V2) {
        let r = point_world - self.pos;

        let perp_vec = get_perp_matrix() * r;

        let f = force.magnitude();
        let i_i = self.i_i;
        self.w = self.w + force.perp(&r) * i_i;
        self.vel = self.vel - force * (force.dot(&r) * self.i_m);
    }
}
