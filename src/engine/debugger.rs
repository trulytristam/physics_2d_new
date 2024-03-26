use lazy_static::lazy_static;
use macroquad::prelude;
use std::{sync::Mutex};
#[derive(Default)]
pub struct Debugger {
    draw_callbacks: Vec<fn()>,
}
lazy_static! {
    pub static ref DEBBUGER: Mutex<Debugger> = Mutex::new(Debugger::default());
}

#[rustfmt::skip]
macro_rules! expand_colors {
    ($context:expr, $color:expr, $($match_color:pat , $col:expr),*) => {
        match $color {
            $(
                $match_color => {
                    $context.draw_callbacks.push(|| prelude::draw_rectangle(0., 0., 50., 50., $col));
                },
            )*
            _ => (),
        }
    };
}
pub enum DebugColor{
    RED,
    BLUE,
    GREEN,
}
impl Debugger {
    pub fn draw_red(&mut self) {
        self.draw_callbacks
            .push(|| prelude::draw_rectangle(0., 0., 50., 50., prelude::RED));
    }
    pub fn draw_box(&mut self, col: DebugColor ) {

        expand_colors!(self, col, DebugColor::RED, prelude::RED,DebugColor::BLUE, prelude::BLUE,DebugColor::GREEN, prelude::GREEN);
        
    }
    // pub fn draw_square(&mut self, color: macroquad::color::Color, pos: u32) {
    //     self.draw_callbacks
    //         .push(|| prelude::draw_rectangle(0., pos as f32 * 50, 50., 50., color.clone()));
    // }

    pub fn draw(&mut self) {
        for f in self.draw_callbacks.iter() {
            f();
        }

        self.draw_callbacks.clear();
    }
}
