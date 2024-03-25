use macroquad::{self};
use std::cell::RefCell;
use std::rc::Rc;

pub mod colliders;
use colliders::Collider;
pub mod render_info;
use render_info::RenderInfo;

pub mod physic_info;
use physic_info::PhysicInfo;

use super::engine_camera::{Conversion, EngineCamera};

use crate::engine::objects::colliders::Circle;

use self::colliders::Poly;

use super::EnginePhysicsInfo;

use super::EngineTime;

type V2 = nalgebra::Vector2<f64>;
type MP<T> = Rc<RefCell<T>>;

#[derive(Clone)]
pub struct ObjectInfo {
    pub vertex_data: Vec<V2>,
    pub render: RenderInfo,
    pub physic: PhysicInfo,
    pub size: f64,
}

impl ObjectInfo {
    fn default() -> Self {
        ObjectInfo {
            vertex_data: vec![],
            render: RenderInfo::default(),
            physic: PhysicInfo::default(),
            size: 70.,
        }
    }
}
#[derive(Clone)]
pub struct Object {
    pub info: ObjectInfo,
    pub collider: Collider,
}

impl Object {
    fn default() -> Self {
        Object {
            info: ObjectInfo::default(),
            collider: Collider::Poly(Poly::default()),
        }
    }
    pub fn update(&mut self, _engine_time: EngineTime, _engine_physics_info: EnginePhysicsInfo) {
        self.integrate(_engine_time, _engine_physics_info);
        self.generate_collider();
    }

    pub fn new_poly_from_vec(points: Vec<V2>) -> MP<Object> {
        let mut temp = Object::default();
        temp.info.vertex_data = points;
        return Rc::new(RefCell::new(temp));
    }

    pub fn translated(&self, v: V2, a: f64) -> MP<Object> {
        let mut temp = self.clone();
        temp.info.physic.pos += v;
        temp.info.physic.ang += a;

        Rc::new(RefCell::new(temp))
    }

    fn integrate(&mut self, _engine_time: EngineTime, _engine_physics_info: EnginePhysicsInfo) {
        let o = &mut self.info.physic;
        o.ang += _engine_time.time_last_frame.as_secs_f64();
        o.pos += o.vel * _engine_time.time_last_frame.as_secs_f64();
    }
    fn generate_collider(&mut self) {
        self.collider = match &self.collider {
            Collider::Poly(_) => {
                let mut temp: Vec<V2> = vec![];
                for p in self.info.vertex_data.iter() {
                    let rot = nalgebra::Rotation2::new(self.info.physic.ang);
                    let new_p = rot.transform_vector(&p) * self.info.size + self.info.physic.pos;
                    temp.push(new_p)
                }
                Collider::Poly(Poly { points: temp })
            }
            Collider::Circle(_) => Collider::Circle(Circle {
                pos: self.info.physic.pos,
                radius: self.info.size,
            }),
        };
    }

    pub fn draw(&self, cam: EngineCamera) {
        match &self.collider {
            Collider::Poly(p) => {
                let mut i = 0;
                for _ in p.points.iter() {
                    let a = p.points[i].world_to_screen(&cam);
                    let b = (p.points[(i + 1) % p.points.len()]).world_to_screen(&cam);
                    if self.info.render.fill {
                        let p_screen = self.info.physic.pos.world_to_screen(&cam);
                        macroquad::shapes::draw_triangle(
                            a.into_vec2(),
                            b.into_vec2(),
                            p_screen.into_vec2(),
                            self.info.render.fill_color,
                        )
                    }
                    macroquad::shapes::draw_line(
                        a.x as f32,
                        a.y as f32,
                        b.x as f32,
                        b.y as f32,
                        2.,
                        self.info.render.line_color,
                    );

                    i += 1;
                }
            }
            Collider::Circle(_) => {
                let o = &self.info.physic;
                let np = o.pos.world_to_screen(&cam);
                macroquad::shapes::draw_circle(
                    np.x as f32,
                    np.y as f32,
                    (self.info.size * cam.scale) as f32,
                    self.info.render.line_color,
                );
            }
        }
    }
}
