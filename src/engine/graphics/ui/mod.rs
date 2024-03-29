#[derive(Default)]
pub struct Ui {
    pub widjets_info: ui_widjet_info::UiWidjetsInfo,
    widjets: Vec<Rc<RefCell<dyn Widjet>>>,
}

impl Ui {
    pub fn update(&mut self) {
        self.delete_flagged_widgets();
    }
    pub fn get_widget_count(&self) -> u32 {
        self.widjets.len() as u32
    }

    pub fn delete_flagged_widgets(&mut self) {
        self.widjets
            .retain(|e| !e.borrow_mut().get_delete());
    }

    pub fn add_widget(&mut self, widjet: Rc<RefCell<dyn Widjet>>) {
        self.widjets.push(widjet);
    }

    pub fn draw(&mut self) {
        for widget in self.widjets.iter() {
            widget.borrow().draw();
        }
    }
    pub fn set_selected_widget(&mut self, id: u32) {
        self.widjets_info.widget_selected = id;
    }

    pub fn press_selected_widget(&mut self, info: Rc<dyn UpdateInfo>) {
        for w in self.widjets.iter_mut() {
            if w.borrow().get_widjet_id() == self.widjets_info.widget_selected
                && self.widjets_info.widget_selected != 0
            {
                // if length > 2 {
                //     DEBBUGER.lock().unwrap().draw_box(DebugColor::BLUE);
                // } else {
                //     DEBBUGER.lock().unwrap().draw_box(DebugColor::RED);
                // }
                w.borrow_mut()
                    .on_hold(info.clone(), None)
            }
        }
    }

    pub fn release_selected_widget(&mut self, info: Rc<dyn UpdateInfo>) {
        for w in self.widjets.iter_mut() {
            if w.borrow().get_widjet_id() == self.widjets_info.widget_selected
                && self.widjets_info.widget_selected != 0
            {
                // DEBBUGER.draw_box(macroquad::prelude::RED);
                w.borrow_mut()
                    .on_release(info.clone(), None)
            }
        }
    }
}

pub mod widjets;
use std::{cell::RefCell, rc::Rc};

use widjets::Widjet;

use crate::engine::engine_camera;

use self::widjets::{ui_widjet_info, UpdateInfo};
