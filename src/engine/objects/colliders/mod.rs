use nalgebra;
type V2 = nalgebra::Vector2<f64>;

#[derive(Clone, Default)]
pub struct Poly {
    pub points: Vec<V2>,
}

#[derive(Clone, Default)]
pub struct Circle {
    pub pos: V2,
    pub radius: f64,
}

#[derive(Clone)]
pub enum Collider {
    Poly(Poly),
    Circle(Circle),
}

impl Default for Collider {
    fn default() -> Self {
        return Collider::Poly(Poly::default());
    }
}

pub struct Manifold {
    collision_normal: V2,
    collision_point: V2,
}

impl Collider {
    fn is_colliding(&self, _other: &Collider) -> Option<Manifold> {
        todo!();
    }

    fn test() {}
}
