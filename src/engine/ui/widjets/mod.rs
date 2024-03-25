pub mod impulse_adder;
use crate::engine::{EngineCamera, V2};

pub mod ui_widjet_info;

pub struct WidjetUpdateInfo {
    v2s: Vec<V2>,
}

struct WidjetInfo {
    delete: bool,
    widjet_id: u32,
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
    fn on_press(&mut self, info: WidjetUpdateInfo, callback: Option<fn() -> ()>);
    fn on_hold(&mut self, info: WidjetUpdateInfo, callback: Option<fn() -> ()>);
    fn on_release(&mut self, info: WidjetUpdateInfo, callback: Option<fn() -> ()>);
    fn get_delete(&self) -> bool;
    fn get_widjet_id(&self) -> u32;
}
