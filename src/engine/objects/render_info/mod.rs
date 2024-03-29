use macroquad::prelude::*;
#[derive(Clone, Default)]
pub struct RenderInfo {
    pub fill_color: macroquad::color::Color,
    pub line_color: macroquad::color::Color,
    pub show_vert: bool,
    pub fill: bool,
}
impl RenderInfo {
    pub fn default() -> Self {
        RenderInfo {
            fill_color: macroquad::color::BLANK,
            line_color: WHITE,
            show_vert: true,
            fill: true,
        }
    }
}
