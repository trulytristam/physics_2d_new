use super::Object;
use macroquad::prelude::*;
use nalgebra::{self};
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

pub trait ConversionV2 {
    fn screen_to_world(&self, cam: &EngineCamera) -> Self;
    fn world_to_screen(&self, cam: &EngineCamera) -> Self;
    fn into_vec2(&self) -> Vec2;
    fn local_to_world(&self, object: MP<Object>) -> V2;
    fn world_to_local(&self, object: MP<Object>) -> V2;
}
pub trait Conversionf32f32 {
    fn into_v2(&self) -> V2;
}

impl ConversionV2 for V2 {
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
        let p = rot.transform_vector(&self) * object.info.poly_size + object.info.physic.pos;
        return p;
    }
    fn world_to_local(&self, object: MP<Object>) -> V2 {
        let object = (*object).borrow();
        let rot = nalgebra::Rotation2::new(-object.info.physic.ang);
        let p = (rot.transform_vector(&self) - object.info.physic.pos) / object.info.poly_size;
        return p;
    }
}

impl Conversionf32f32 for (f32, f32) {
    fn into_v2(&self) -> V2 {
        return V2::new(self.0 as f64, self.1 as f64);
    }
}
