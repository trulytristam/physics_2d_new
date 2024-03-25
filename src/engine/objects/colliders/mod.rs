use nalgebra;
type V2 = nalgebra::Vector2<f64>;
use std::cell::RefCell;
use std::rc::Rc;

use crate::engine::helper_functions;

use super::Object;

type MP<T> = Rc<RefCell<T>>;

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
    a: MP<Object>,
    b: MP<Object>,
    collision_normal: V2,
    collision_point: V2,
}

impl Collider {
    pub fn is_colliding(&self, _other: &Collider) -> Option<Manifold> {
        todo!();
    }

    pub fn point_inside(&self, point: &V2) -> bool {
        match self {
            Collider::Poly(p) => p.point_inside(point),
            Collider::Circle(c) => c.point_inside(point),
        }
    }
}

impl Poly {
    fn point_inside(&self, point: &V2) -> bool {
        for i in 0..self.points.len() {
            let a = self.points[i];
            let b = self.points[(i + 1) % self.points.len()];
            let v = b - a;
            let o = point - a;
            let n = helper_functions::get_perp_matrix() * v;

            if o.dot(&n) > 0. {
                return false;
            }
        }
        return true;
    }
}
impl Circle {
    fn point_inside(&self, point: &V2) -> bool {
        return (point - self.pos).magnitude() < self.radius;
    }
}
