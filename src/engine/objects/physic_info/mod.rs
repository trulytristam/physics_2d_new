use nalgebra;

type V2 = nalgebra::Vector2<f64>;

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
