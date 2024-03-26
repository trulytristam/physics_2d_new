pub mod impulse_adder;
use std::{marker::PhantomData, rc::Rc};

use crate::engine::{objects::V2, EngineCamera};

pub mod ui_widjet_info;

struct WidjetInfo {
    delete: bool,
    widjet_id: u32,
}

struct Info<T> {
    t: PhantomData<T>,
}

pub struct ImpulseAdderInfo {
    pub mouse: V2,
}
impl UpdateInfo for ImpulseAdderInfo {}

pub trait UpdateInfo {
    fn to_vec(&self) -> Vec<f64> {
        todo!();
    }
}
impl WidjetInfo {
    fn default(widjet_id: u32) -> Self {
        WidjetInfo {
            delete: false,
            widjet_id,
        }
    }
}

pub trait Widjet {
    fn draw(&self, cam: &EngineCamera);
    fn on_press(&mut self, info: Rc<dyn UpdateInfo>, callback: Option<fn() -> ()>);
    fn on_hold(&mut self, info: Rc<dyn UpdateInfo>, callback: Option<fn() -> ()>);
    fn on_release(&mut self, info: Rc<dyn UpdateInfo>, callback: Option<fn() -> ()>);
    fn get_delete(&self) -> bool;
    fn get_widjet_id(&self) -> u32;
}
