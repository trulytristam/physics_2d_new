#![allow(dead_code)]

use crate::engine::debugger::{Debg, DEBBUGER};
use crate::engine::engine_camera::ConversionV2;
use crate::engine::helper_functions::{gen_perp_matrix, point_inside_shape};
use crate::engine::objects::colliders::Poly;
use crate::engine::objects::{colliders::Collider, V2};
use macroquad::color::{BLUE, GREEN, RED};
use macroquad::prelude::rand;

#[derive(Debug)]
pub struct Simplex {
    minko_points: Vec<MinkowPoint>,
    dir: V2,
    collision_delta: f64,
    collision_found: bool,
}
impl Default for Simplex {
    fn default() -> Self {
        let x: f64 = rand::gen_range(-1., 1.);
        let y: f64 = rand::gen_range(-1., 1.);
        let x = 1.;
        let y = 1.;
        Simplex {
            minko_points: Vec::new(),
            dir: V2::new(x, y).normalize(),
            collision_delta: 0.001,
            collision_found: false,
        }
    }
}

#[derive(Debug)]
enum ParentCount {
    One(MinkowPoint),
    Two(
        (
            MinkowPoint,
            MinkowPoint,
        ),
        f64,
    ),
}
#[derive(Debug)]
pub struct ClosestPoint {
    point: V2,
    parent: ParentCount,
}
impl ClosestPoint {
    fn get_new_dir(&self) -> V2 {
        (self.point * -1.).normalize()
    }

    pub fn closest_on_shapes(&self, _a: &Collider, _b: &Collider) -> (V2, V2) {
        match &self.parent {
            ParentCount::One(p) => {
                let a = _a.get_point(p.p, p.a);
                let b = _b.get_point(p.p, p.b);
                return (a, b);
            }
            ParentCount::Two((p1, p2), inter) => {
                let a1 = _a.get_point(p1.p, p1.a);
                let a2 = _a.get_point(p2.p, p2.a);
                let b1 = _b.get_point(p1.p, p1.b);
                let b2 = _b.get_point(p2.p, p2.b);

                return (
                    interpolate(a1, a2, *inter),
                    interpolate(b1, b2, *inter),
                );
            }
        }
    }
}
fn interpolate(a: V2, b: V2, inter: f64) -> V2 {
    return a + (b - a) * inter;
}
///bool in terminate represents if collision present
#[derive(Debug)]
pub enum GjkInstruction {
    Continue,
    Terminate(bool),
}

#[derive(Debug)]
pub struct EvolveResult {
    gjk_instruction: GjkInstruction,
    pub closest_point: Option<ClosestPoint>,
}

impl EvolveResult {
    fn continue_() -> EvolveResult {
        return EvolveResult {
            gjk_instruction: GjkInstruction::Continue,
            closest_point: None,
        };
    }
    fn terminate_(is_colliding: bool, closest_point: Option<ClosestPoint>) -> EvolveResult {
        return EvolveResult {
            gjk_instruction: GjkInstruction::Terminate(is_colliding),
            closest_point,
        };
    }
}

impl Simplex {
    pub fn draw(&self, _a: &Collider, _b: &Collider) {
        let colors = vec![
            macroquad::prelude::RED,
            macroquad::prelude::GREEN,
            macroquad::prelude::BLUE,
        ];
        DEBBUGER.draw_poly(
            self.minko_points
                .iter()
                .map(|p| p.p)
                .collect(),
            Some(&colors),
            macroquad::prelude::BLACK,
        );
        // draw points on colliders
        match (_a, _b) {
            (Collider::Poly(pa), Collider::Poly(pb)) => {
                let mut i = 0;
                for p in self.minko_points.iter() {
                    let a = pa.points[p.a.unwrap()];
                    let b = pb.points[p.b.unwrap()];
                    DEBBUGER.draw_dot(
                        a.world_to_screen(),
                        colors[i],
                    );
                    DEBBUGER.draw_dot(
                        b.world_to_screen(),
                        colors[i],
                    );
                    DEBBUGER.draw_line(
                        a.world_to_screen(),
                        b.world_to_screen(),
                        colors[i],
                    );
                    i += 1;
                }
            }
            (..) => (),
        }
    }
    pub fn evolve_simplex(&mut self, p: MinkowPoint) -> EvolveResult {
        let mut closest = self.closest_point_to_origin();
        if !self.candidate_valid(&p) {
            if self.collision_found {
                return EvolveResult::terminate_(true, closest);
            }
            return EvolveResult::terminate_(false, closest);
        }
        //grow simplex
        let out = match self.minko_points.len() {
            //test
            0 => {
                self.minko_points.push(p);
                EvolveResult::continue_()
            }
            1 => {
                self.minko_points.push(p);
                EvolveResult::continue_()
            }
            2 => {
                self.push_arrange_clockwise(p);
                let shape = self
                    .minko_points
                    .iter()
                    .map(|p| p.p)
                    .collect();
                if point_inside_shape(
                    &shape,
                    &V2::new(0., 0.),
                ) {
                    self.collision_found = true;
                    return EvolveResult::terminate_(true, closest);
                }
                EvolveResult::continue_()
            }
            3 => {
                self.clean_simplex();
                self.push_arrange_clockwise(p);
                let shape = self
                    .minko_points
                    .iter()
                    .map(|p| p.p)
                    .collect();
                if point_inside_shape(
                    &shape,
                    &V2::new(0., 0.),
                ) {
                    self.collision_found = true;
                    return EvolveResult::terminate_(true, closest);
                }
                EvolveResult::continue_()
            }

            _ => unreachable!("gjk has to many points: from evolve_simplex()"),
        };

        //find closest point on minkowdiff to origin
        //also asign new direction towards origin
        closest = self.closest_point_to_origin();
        self.dir = closest.as_ref().unwrap().get_new_dir();
        if self.minko_points.len() > 0
            && closest
                .as_ref()
                .unwrap()
                .point
                .magnitude()
                < 0.001
        {
            self.collision_found = true;
            return EvolveResult::terminate_(true, closest);
        }
        return out;
    }

    fn candidate_valid(&self, candidate: &MinkowPoint) -> bool {
        let delta = 0.00001;
        for p in self.minko_points.iter() {
            if (p.p - candidate.p).magnitude() < delta {
                return false;
            }
        }
        return true;
    }
    fn push_arrange_clockwise(&mut self, p: MinkowPoint) {
        assert!(self.minko_points.len() == 2);
        let a = self.minko_points[0].p;
        let b = self.minko_points[1].p;
        let v = b - a;
        let v_perp = gen_perp_matrix() * v;
        let dot = v_perp.dot(&p.p);
        if dot < 1. {
            // println!("clockwise");
            self.minko_points.push(p);
        } else {
            // println!("anticlockwise");
            let temp = self.minko_points.remove(1);
            self.minko_points.push(p);
            self.minko_points.push(temp);
        }
    }
    ///returns none if trying to get closest when simple has no points
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
        // works
        assert!(self.minko_points.len() == 3);
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
        // println!("n Points {}", self.minko_points.len());
    }
}
fn closest_to_line(a: &MinkowPoint, b: &MinkowPoint) -> ClosestPoint {
    //works
    let v = b.p - a.p;
    let v_n = v.normalize();
    let a_i = a.p * -1.;
    let d = a_i.dot(&v_n) / v.magnitude();
    let inter = d;
    let p = a.p + v * inter;
    if inter > 0. && inter < 1. {
        ClosestPoint {
            point: p,
            parent: ParentCount::Two(
                (a.clone(), b.clone()),
                inter,
            ),
        }
    } else if inter <= 0. {
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

pub fn closest_to_tri(points: [&MinkowPoint; 3]) -> ClosestPoint {
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
    let b_sup = _b.get_support(&(dir * -1.));
    MinkowPoint {
        p: a_sup.point - b_sup.point,
        a: a_sup.index,
        b: b_sup.index,
    }
}

pub struct GjkResult {
    pub is_colliding: bool,
    pub closest_point: Option<ClosestPoint>,
    pub simplex: Simplex,
}
#[allow(unreachable_code)]
pub fn gjk(_a: &Collider, _b: &Collider) -> GjkResult {
    let mut simp = Simplex::default();

    for _ in 0..1000 {
        let minkow_point = get_minko_point(simp.dir, &_a, &_b);
        let evolve_result = simp.evolve_simplex(minkow_point);
        let gjk_instruction = evolve_result.gjk_instruction;
        match gjk_instruction {
            GjkInstruction::Continue => continue,
            GjkInstruction::Terminate(is_colliding) => {
                return GjkResult {
                    is_colliding,
                    closest_point: evolve_result.closest_point,
                    simplex: simp,
                }
            }
        }
    }

    return GjkResult {
        is_colliding: false,
        closest_point: None,
        simplex: simp,
    };
}

///MinkowPoint is in world space
#[derive(Debug, Clone)]
pub struct MinkowPoint {
    p: V2,
    a: Option<usize>,
    b: Option<usize>,
}
impl MinkowPoint {
    fn new_from_v2(p: V2) -> Self {
        MinkowPoint {
            p,
            a: None,
            b: None,
        }
    }
}

#[cfg(test)]
mod gjk_tests {

    use crate::engine::{collisions::gjk::closest_to_tri, objects::V2};

    use super::{closest_to_line, MinkowPoint, Simplex};

    #[test]
    fn test_simplex() {
        let mut s = Simplex::default();
        let a = MinkowPoint::new_from_v2(V2::new(-2., 0.));
        let b = MinkowPoint::new_from_v2(V2::new(0., 2.));
        let c = MinkowPoint::new_from_v2(V2::new(2., 0.));

        let result_a = s.evolve_simplex(a);
        println!(
            "result a {:?}",
            result_a
        );
        println!(
            "new dir a {:?}\n",
            s.dir
        );
        let result_b = s.evolve_simplex(b);
        println!(
            "result b {:?}",
            result_b
        );
        println!(
            "new dir b {:?} \n",
            s.dir
        );
        let result_c = s.evolve_simplex(c);
        println!(
            "result c {:?}",
            result_c
        );
        println!(
            "new dir c {:?} \n",
            s.dir
        );

        //-----------------
        let d = MinkowPoint::new_from_v2(V2::new(2., -2.));
        let result_d = s.evolve_simplex(d);
        println!(
            "result d {:?}",
            result_d
        );

        println!("simplex: {:?}", s);
        panic!();
    }

    #[test]
    fn test_simplex_closest_to_line() {
        let a = MinkowPoint::new_from_v2(V2::new(-2., 0.));
        let b = MinkowPoint::new_from_v2(V2::new(0., 2.));
        let result = closest_to_line(&a, &b);
        println!("()()()()()()()()");
        println!(
            "closest to line result {:?}",
            result
        );
        println!("()()()()()()()()");
        panic!();
    }
    #[test]
    fn test_simplex_closest_to_tri() {
        let a = MinkowPoint::new_from_v2(V2::new(-2., 0.));
        let b = MinkowPoint::new_from_v2(V2::new(0., 2.));
        let c = MinkowPoint::new_from_v2(V2::new(-0.5, 0.5));
        let result = closest_to_tri([&a, &b, &c]);
        println!("()()()()()()()()");
        println!(
            "closest to tri result {:?}",
            result
        );
        println!("()()()()()()()()");
        panic!();
    }
}
