use nalgebra;
type V2 = nalgebra::Vector2<f64>;

#[derive(Clone)]
pub struct EngineCamera {
    pub pos: V2,
    pub scale: f64,
}
impl Default for EngineCamera {
    fn default() -> Self {
        EngineCamera {
            pos: V2::new(
                -macroquad::window::screen_width() as f64 / 2.,
                -macroquad::window::screen_height() as f64 / 2.,
            ),
            scale: 1.,
        }
    }
}

pub trait Conversion {
    fn screen_to_world(&self, cam: &EngineCamera) -> Self;
    fn world_to_screen(&self, cam: &EngineCamera) -> Self;
}

impl Conversion for V2 {
    fn screen_to_world(&self, cam: &EngineCamera) -> Self {
        self / cam.scale + cam.pos
    }
    fn world_to_screen(&self, cam: &EngineCamera) -> Self {
        (self - cam.pos) * cam.scale
    }
}
