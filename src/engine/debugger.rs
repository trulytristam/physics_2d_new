extern crate proc_macro;
use lazy_static::lazy_static;
use macroquad::prelude;
use std::sync::{Arc, Mutex};

use super::objects::V2;
#[derive(Default)]
pub struct Debugger {
    draw_callbacks: Vec<Arc<dyn Fn() + Send + Sync>>,
    pos: u32,
}
lazy_static! {
    pub static ref DEBBUGER: Mutex<Debugger> = Mutex::new(Debugger::default());
}

#[rustfmt::skip]
pub enum DebugColor {
    RED,
    BLUE,
    GREEN,
}

pub trait Debg {
    fn draw_box(&self, col: prelude::Color);
    fn draw_line(&self, a_screen: V2, b_screen: V2, col: prelude::Color);
    fn draw_arrow(&self, a_screen: V2, b_screen: V2, col: prelude::Color);
    fn draw_dot(&self, pos_screen: V2, col: prelude::Color);
    fn draw(&self);
    fn draw_text(&self, text: &str, pos_screen: V2, col: prelude::Color);
}

impl Debg for Mutex<Debugger> {
    fn draw_box(&self, col: prelude::Color) {
        let mut g = self.lock().unwrap();
        let col = col.clone();
        let pos = g.pos;
        g.add_callback(Arc::new(move || {
            prelude::draw_rectangle(0., 50. * pos as f32, 50., 50., col)
        }));
        g.pos += 1;
    }
    fn draw_line(&self, a_screen: V2, b_screen: V2, col: prelude::Color) {
        let mut g = self.lock().unwrap();
        let col = col.clone();
        g.add_callback(Arc::new(move || {
            prelude::draw_line(
                a_screen.x as f32,
                a_screen.y as f32,
                b_screen.x as f32,
                b_screen.y as f32,
                4.,
                col,
            );
        }));
    }
    fn draw_arrow(&self, a_screen: V2, b_screen: V2, col: prelude::Color) {
        let mut g = self.lock().unwrap();
        let col = col.clone();
        let v = (a_screen - b_screen).normalize() * 20.;
        let rot1 = nalgebra::Rotation2::new(std::f64::consts::PI / 6.);
        let rot2 = nalgebra::Rotation2::new(std::f64::consts::PI / -6.);
        let a1 = b_screen + rot1 * v;
        let a2 = b_screen + rot2 * v;
        g.add_callback(Arc::new(move || {
            prelude::draw_line(
                a_screen.x as f32,
                a_screen.y as f32,
                b_screen.x as f32,
                b_screen.y as f32,
                4.,
                col,
            );
            prelude::draw_line(
                a1.x as f32,
                a1.y as f32,
                b_screen.x as f32,
                b_screen.y as f32,
                4.,
                col,
            );
            prelude::draw_line(
                a2.x as f32,
                a2.y as f32,
                b_screen.x as f32,
                b_screen.y as f32,
                4.,
                col,
            );
        }));
    }
    fn draw_dot(&self, pos_screen: V2, col: prelude::Color) {
        let mut g = self.lock().unwrap();
        let col = col.clone();
        g.add_callback(Arc::new(move || {
            prelude::draw_circle(pos_screen.x as f32, pos_screen.y as f32, 5., col);
        }));
    }
    fn draw_text(&self, text: &str, pos_screen: V2, col: prelude::Color) {
        let mut g = self.lock().unwrap();
        let col = col.clone();
        let text = text.to_owned();
        let pos = pos_screen.to_owned();
        g.add_callback(Arc::new(move || {
            prelude::draw_text(&text, pos.x as f32, pos.y as f32, 40., col);
        }));
    }

    fn draw(&self) {
        let mut g = self.lock().unwrap();
        for f in g.draw_callbacks.iter() {
            f();
        }
        g.init();
    }
}
impl Debugger {
    pub fn add_callback(&mut self, c: Arc<dyn Fn() + Send + Sync>) {
        self.draw_callbacks.push(c);
    }

    pub fn init(&mut self) {
        self.draw_callbacks.clear();
        self.pos = 0;
    }
}
