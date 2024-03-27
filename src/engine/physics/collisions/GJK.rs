#![allow(dead_code)]
use crate::engine::helper_functions::{gen_perp_matrix, point_inside_shape};
use crate::engine::objects::{colliders::Collider, V2};
use macroquad::prelude::rand;

struct Simplex {
    minko_points: Vec<MinkowPoint>,
    dir: V2,
}
impl Default for Simplex {
    fn default() -> Self {
        let x: f64 = rand::gen_range(-1., 1.);
        let y: f64 = rand::gen_range(-1., 1.);
        Simplex {
            minko_points: Vec::new(),
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
impl ClosestPoint {
    fn get_new_dir(&self) -> V2 {
        (self.point * -1.).normalize()
    }
}
struct SimplexEvolveResult {
    point_inside_simplex: bool,
}
impl Simplex {
    fn evolve_simplex(&mut self, p: MinkowPoint) -> SimplexEvolveResult {
        let closest = self.closest_point_to_origin();
        if let Some(closest) = closest {
            self.dir = closest.get_new_dir();
        }
        match self.minko_points.len() {
            //test
            0 => {
                self.minko_points.push(p);
                SimplexEvolveResult {
                    point_inside_simplex: false,
                }
            }
            1 => {
                self.minko_points.push(p);
                SimplexEvolveResult {
                    point_inside_simplex: false,
                }
            }
            2 => {
                self.push_arrange_clockwise(p);
                SimplexEvolveResult {
                    point_inside_simplex: false,
                }
            }
            3 => {
                let shape = self.minko_points.iter().map(|p| p.p).collect();
                if point_inside_shape(&shape, &V2::new(0., 0.)) {
                    SimplexEvolveResult {
                        point_inside_simplex: true,
                    }
                } else {
                    self.clean_simplex();
                    self.push_arrange_clockwise(p);
                    SimplexEvolveResult {
                        point_inside_simplex: false,
                    }
                }
            }

            _ => SimplexEvolveResult {
                point_inside_simplex: false,
            },
        }
    }

    fn push_arrange_clockwise(&mut self, p: MinkowPoint) {
        assert!(self.minko_points.len() == 2);

        let a = self.minko_points[0].p;
        let b = self.minko_points[1].p;

        let v = b - a;
        let v_perp = gen_perp_matrix() * v;

        let dot = v_perp.dot(&p.p);

        if dot < 1. {
            self.minko_points.push(p);
        } else {
            let temp = self.minko_points.remove(1);
            self.minko_points.push(p);
            self.minko_points.push(temp);
        }
    }

    fn closest_point_to_origin(&self) -> Option<ClosestPoint> {
        match self.minko_points.len() {
            1 => Some(ClosestPoint {
                point: self.minko_points[0].p,
                parent: ParentCount::One(self.minko_points[0].clone()),
            }),
            2 => Some(closest_to_line(
                &self.minko_points[0],
                &self.minko_points[1],
            )),
            3 => Some(closest_to_tri([
                &self.minko_points[0],
                &self.minko_points[1],
                &self.minko_points[2],
            ])),
            _ => None,
        }
    }

    ///removes furthest point from origin
    fn clean_simplex(&mut self) {
        let dir = self.dir * -1.;
        let mut furthest: Option<f64> = None;
        let mut furthest_i = 0;

        let mut i = 0;
        for point in self.minko_points.iter() {
            let dist = dir.dot(&point.p);
            if furthest.is_none() || dist > furthest.unwrap() {
                furthest = Some(dist);
                furthest_i = i;
            }
            i += 1;
        }
        self.minko_points.remove(furthest_i);
    }
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

#[allow(unreachable_code)]
fn gjk(_a: Collider, _b: Collider) -> bool {
    let mut simp = Simplex::default();

    loop {
        let minkow_point = get_minko_point(simp.dir, &_a, &_b);
        let evolve_result = simp.evolve_simplex(minkow_point);

        if evolve_result.point_inside_simplex {
            break;
        }
    }
    todo!();
}

#[derive(Clone)]
struct MinkowPoint {
    p: V2,
    a: Option<usize>,
    b: Option<usize>,
}
