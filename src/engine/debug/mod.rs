use macroquad::prelude;
use std::cell::RefCell;
#[derive(Default)]
pub struct Debugger {
    draw_callbacks: RefCell<Vec<fn()>>,
}

impl Debugger {
    pub fn draw_red(&self) {
        self.draw_callbacks
            .borrow_mut()
            .push(|| prelude::draw_rectangle(0., 0., 50., 50., prelude::RED));
    }

    pub fn draw(&mut self) {
        for f in self.draw_callbacks.borrow().iter() {
            f();
        }

        self.draw_callbacks.borrow_mut().clear();
    }
}
