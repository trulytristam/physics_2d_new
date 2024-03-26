#[derive(Default)]
pub struct Ui {
    pub widjets_info: ui_widjet_info::UiWidjetsInfo,
    widjets: Vec<Rc<RefCell<dyn Widjet>>>,
}

impl Ui {
    pub fn update(&mut self) {
        self.delete_flagged_widgets();
    }

    pub fn delete_flagged_widgets(&mut self) {
        self.widjets.retain(|e| !e.borrow_mut().get_delete());
    }

    pub fn add_widget(&mut self, widjet: Rc<RefCell<dyn Widjet>>) {
        self.widjets.push(widjet);
    }

    pub fn draw(&mut self, cam: &engine_camera::EngineCamera) {
        for widget in self.widjets.iter() {
            widget.borrow().draw(cam);
        }
    }
    pub fn set_selected_widget(&mut self, id: u32) {
        self.widjets_info.widget_selected = id;
    }

    pub fn press_selected_widget(&mut self, info: Rc<dyn UpdateInfo>) {
        for w in self.widjets.iter_mut() {
            if w.borrow().get_widjet_id() == self.widjets_info.widget_selected {
                w.borrow_mut().on_press(info.clone(), None)
            }
        }
    }

    pub fn release_selected_widget(&mut self) {}
}

pub mod widjets;
use std::{cell::RefCell, rc::Rc};

use widjets::Widjet;

use crate::engine::engine_camera;

use self::widjets::{ui_widjet_info, UpdateInfo};
