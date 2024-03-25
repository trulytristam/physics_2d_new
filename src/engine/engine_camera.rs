use super::{objects, Object};
use macroquad::prelude::*;
use nalgebra::{self, AbstractRotation};
type V2 = nalgebra::Vector2<f64>;
use std::cell::RefCell;
use std::rc::Rc;
type MP<T> = Rc<RefCell<T>>;

#[derive(Clone)]
pub struct EngineCamera {
    pub pos: V2,
    pub scale: f64,
}
impl Default for EngineCamera {
    fn default() -> Self {
        EngineCamera {
            pos: V2::new(
                -macroquad::window::screen_width() as f64 / 2.,
                -macroquad::window::screen_height() as f64 / 2.,
            ),
            scale: 1.,
        }
    }
}

pub trait Conversion {
    fn screen_to_world(&self, cam: &EngineCamera) -> Self;
    fn world_to_screen(&self, cam: &EngineCamera) -> Self;
    fn into_vec2(&self) -> Vec2;
    fn local_to_world(&self, object: MP<Object>) -> V2;
    fn world_to_local(&self, object: MP<Object>) -> V2;
}

impl Conversion for V2 {
    fn screen_to_world(&self, cam: &EngineCamera) -> Self {
        self / cam.scale + cam.pos
    }
    fn world_to_screen(&self, cam: &EngineCamera) -> Self {
        (self - cam.pos) * cam.scale
    }
    fn into_vec2(&self) -> Vec2 {
        Vec2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
    fn local_to_world(&self, object: MP<Object>) -> V2 {
        let object = (*object).borrow();
        let rot = nalgebra::Rotation2::new(object.info.physic.ang);
        let p = rot.transform_vector(&self) * object.info.size + object.info.physic.pos;
        return p;
    }
    fn world_to_local(&self, object: MP<Object>) -> V2 {
        let object = (*object).borrow();
        let rot = nalgebra::Rotation2::new(-object.info.physic.ang);
        let p = (rot.transform_vector(&self) - object.info.physic.pos) / object.info.size;
        return p;
    }
}
