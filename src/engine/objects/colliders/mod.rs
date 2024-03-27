use nalgebra;
type V2 = nalgebra::Vector2<f64>;

use std::cell::RefCell;
use std::rc::Rc;

use crate::engine::helper_functions::{self, point_inside_shape};

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
pub struct SupportInfo {
    pub index: Option<usize>,
    pub point: V2,
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

    pub fn get_support(&self, dir_unit: &V2) -> SupportInfo {
        match self {
            Collider::Poly(poly) => {
                let mut furthest: f64 = 0.;
                let mut furthest_point: Option<V2> = None;
                let mut furthest_i = 0;

                let mut i = 0;
                for p in poly.points.iter() {
                    let d = dir_unit.dot(&p);

                    if d > furthest || furthest_point.is_none() {
                        furthest = d;
                        furthest_point = Some(p.clone());
                        furthest_i = i;
                    }
                    i += 1;
                }

                SupportInfo {
                    index: Some(furthest_i),
                    point: furthest_point.unwrap(),
                }
            }
            Collider::Circle(circle) => SupportInfo {
                index: None,
                point: circle.pos + dir_unit * circle.radius,
            },
        }
    }
}

impl Poly {
    fn point_inside(&self, point: &V2) -> bool {
        point_inside_shape(&self.points, point)
    }
}
impl Circle {
    fn point_inside(&self, point: &V2) -> bool {
        return (point - self.pos).magnitude() < self.radius;
    }
}
