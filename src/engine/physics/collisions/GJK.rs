use crate::engine::objects::{colliders::Collider, V2};
use macroquad::math::Vec3;
use macroquad::prelude::rand;

struct Simplex {
    points: Vec<MinkowPoint>,
    dir: V2,
}
impl Default for Simplex {
    fn default() -> Self {
        let x: f64 = rand::gen_range(-1., 1.);
        let y: f64 = rand::gen_range(-1., 1.);
        Simplex {
            points: Vec::new(),
            dir: V2::new(x, y).normalize(),
        }
    }
}

enum ParentCount {
    One(MinkowPoint),
    Two((MinkowPoint, MinkowPoint), f64),
}
struct ClosestPoint {
    point: V2,
    parent: ParentCount,
}
struct SimplexEvolveResult {
    point_inside_simplex: bool,
}
impl Simplex {
    fn evolve_simplex(&mut self, p: MinkowPoint) -> SimplexEvolveResult {
        match self.points.len() {
            0 => {
                self.points.push(p);
                SimplexEvolveResult {
                    point_inside_simplex: false,
                }
            }
            1 => {
                self.points.push(p);
                SimplexEvolveResult {
                    point_inside_simplex: false,
                }
            }
            2 => {
                self.points.push(p);
                SimplexEvolveResult {
                    point_inside_simplex: false,
                }
            }
            3 => {
                todo!();
            }

            _ => SimplexEvolveResult {
                point_inside_simplex: false,
            },
        }
    }

    fn closest_point_to_origin(&self) -> Option<ClosestPoint> {
        match self.points.len() {
            1 => Some(ClosestPoint {
                point: self.points[0].p,
                parent: ParentCount::One(self.points[0].clone()),
            }),
            2 => Some(closest_to_line(&self.points[0], &self.points[1])),
            3 => Some(closest_to_tri([
                &self.points[0],
                &self.points[1],
                &self.points[2],
            ])),
            _ => None,
        }
    }

    fn clean_simplex(&mut self) {
        //remove furthest point from origin
    }

    fn new_dir(&mut self) {}
    fn random_dir(&mut self) {}
}
fn closest_to_line(a: &MinkowPoint, b: &MinkowPoint) -> ClosestPoint {
    let v = b.p - a.p;
    let a_i = a.p * -1.;
    let v_n = v.normalize();
    let d = a_i.dot(&v_n);
    let inter = d / v.magnitude();
    let p = a.p + v_n * inter;
    if inter > 0. && inter < 1. {
        ClosestPoint {
            point: p,
            parent: ParentCount::Two((a.clone(), b.clone()), inter),
        }
    } else if inter < 0. {
        ClosestPoint {
            point: a.p.clone(),
            parent: ParentCount::One(a.clone()),
        }
    } else {
        ClosestPoint {
            point: b.p.clone(),
            parent: ParentCount::One(b.clone()),
        }
    }
}

fn closest_to_tri(points: [&MinkowPoint; 3]) -> ClosestPoint {
    let mut closest = None;
    let mut min_dist: Option<f64> = None;
    for i in 0..3 {
        let a = points[i];
        let b = points[(i + 1) % 3];
        let c = closest_to_line(a, b);
        let d = c.point.magnitude();

        if closest.is_none() || d < min_dist.unwrap() {
            closest = Some(c);
            min_dist = Some(d);
        }
    }
    closest.unwrap()
}

fn get_minko_point(dir: V2, _a: &Collider, _b: &Collider) -> MinkowPoint {
    let a_sup = _a.get_support(&dir);
    let b_sup = _b.get_support(&dir);
    MinkowPoint {
        p: a_sup.point - b_sup.point,
        a: a_sup.index,
        b: b_sup.index,
    }
}

fn gjk(_a: Collider, _b: Collider) -> bool {
    todo!();
}

#[derive(Clone)]
struct MinkowPoint {
    p: V2,
    a: Option<usize>,
    b: Option<usize>,
}
