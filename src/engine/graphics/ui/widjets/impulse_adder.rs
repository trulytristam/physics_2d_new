use std::rc::Rc;

use super::{UpdateInfo, Widjet, WidjetInfo};
use crate::engine::{
    debugger::{Debg, DebugColor, DEBBUGER},
    engine_camera::{ConversionV2, EngineCamera, CAMERA},
    objects::{Object, MP, V2},
};
use macroquad::prelude::*;

pub struct ImpulseAdder {
    object: MP<Object>,
    point_local: V2,
    point_mouse: V2,
    widjet_info: WidjetInfo,
}

#[allow(unused_variables)]
impl Widjet for ImpulseAdder {
    fn draw(&self) {
        let a = self
            .point_local
            .local_to_world(self.object.clone());
        let a = a.world_to_screen().into_vec2();
        let b = self
            .point_mouse
            .world_to_screen()
            .into_vec2();
        draw_line(
            a.x, a.y, b.x, b.y, 3., WHITE,
        );
    }

    fn on_press(&mut self, info: Rc<dyn UpdateInfo>, callback: Option<fn() -> ()>) {}
    fn on_hold(&mut self, info: Rc<dyn UpdateInfo>, callback: Option<fn() -> ()>) {
        let info_data = info.to_vec();
        if info_data.len() > 0 {
            self.point_mouse = V2::new(
                info_data[0],
                info_data[1],
            );
        }
    }
    fn on_release(&mut self, info: Rc<dyn UpdateInfo>, callback: Option<fn() -> ()>) {
        if let Some(c) = callback {
            c();
        } else {
            let force = self.point_mouse
                - self
                    .point_local
                    .local_to_world(self.object.clone());
            let applied_point = self
                .point_local
                .local_to_world(self.object.clone());
            (*self.object)
                .borrow_mut()
                .info
                .physic
                .apply_impulse(
                    applied_point,
                    force * 3.,
                );
        }

        // DEBBUGER.draw_box(macroquad::prelude::GREEN);

        self.widjet_info.delete = true;
    }
    fn get_delete(&self) -> bool {
        return self.widjet_info.delete;
    }

    fn get_widjet_id(&self) -> u32 {
        return self.widjet_info.widjet_id;
    }
}

impl ImpulseAdder {
    pub fn new(object: MP<Object>, mouse_world: V2, id: u32) -> Self {
        ImpulseAdder {
            object: object.clone(),
            point_local: mouse_world.world_to_local(object),
            point_mouse: mouse_world,
            widjet_info: WidjetInfo::default(id),
        }
    }
}
