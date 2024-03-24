use macroquad::prelude::*;
#[derive(Default)]
pub struct RenderInfo {
    pub fill_color: macroquad::color::Color,
    pub line_color: macroquad::color::Color,
    pub show_vert: bool,
}
impl RenderInfo {
    pub fn default() -> Self {
        RenderInfo {
            fill_color: RED,
            line_color: WHITE,
            show_vert: true,
        }
    }
}
