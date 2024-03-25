use lazy_static::lazy_static;
use macroquad::prelude;
use std::sync::Mutex;
#[derive(Default)]
pub struct Debugger {
    draw_callbacks: Vec<fn()>,
}
lazy_static! {
    pub static ref DEBBUGER: Mutex<Debugger> = Mutex::new(Debugger::default());
}

impl Debugger {
    pub fn draw_red(&mut self) {
        self.draw_callbacks
            .push(|| prelude::draw_rectangle(0., 0., 50., 50., prelude::RED));
    }

    pub fn draw(&mut self) {
        for f in self.draw_callbacks.iter() {
            f();
        }

        self.draw_callbacks.clear();
    }
}
