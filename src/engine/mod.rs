pub struct Engine {
    objects: Vec<MP<Object>>,
    selected_obejct: usize,
    engine_time: EngineTime,
    engine_physics_info: EnginePhysicsInfo,
    ui: ui::Ui,
}

impl Engine {
    pub fn new() -> Self {
        let temp = Engine::default();
        temp
    }

    pub fn default() -> Self {
        Engine {
            objects: vec![
                Object::new_poly_from_vec(vec![
                    V2::new(-1., 1.),
                    V2::new(0., 2.),
                    V2::new(1., 1.),
                    V2::new(0., -2.),
                ])
                .borrow_mut()
                .offset_local_data(V2::new(0., -0.57))
                .translated(
                    V2::new(100., 0.),
                    1.,
                ),
                Object::new_poly_from_vec(vec![
                    V2::new(-1., 1.),
                    V2::new(0., 2.),
                    V2::new(1., 1.),
                    V2::new(0., -2.),
                ])
                .borrow_mut()
                .offset_local_data(V2::new(0., -0.57))
                .translated(
                    V2::new(-100., 0.),
                    1.,
                ),
            ],
            selected_obejct: 0,
            engine_time: EngineTime::default(),
            engine_physics_info: EnginePhysicsInfo::default(),
            ui: ui::Ui::default(),
        }
    }

    pub async fn start(&mut self) {
        loop {
            self.user_input();
            self.update();
            self.draw();
            macroquad::prelude::next_frame().await
        }
    }

    fn update(&mut self) {
        self.engine_time.frame_start();
        self.ui.update();
        self.handle_selected_object();
        for object in self.objects.iter_mut() {
            object.borrow_mut().update(
                self.engine_time.clone(),
                self.engine_physics_info.clone(),
            );
        }
        self.engine_physics_info
            ._collisions
            .generate_pairs(&self.objects);

        for pair in self
            .engine_physics_info
            ._collisions
            .pairs
            .iter()
        {
            let n = self
                .engine_physics_info
                ._collisions
                .count_pairs();
            let text = format!(
                "number of pairs {}",
                n
            );
            let text = text.as_str();
            DEBBUGER.draw_text(
                text,
                V2::new(40., 40.),
                macroquad::color::RED,
            );
        }
    }

    fn user_input(&mut self) {
        if_space_pressed(self);
        if_space_held(self);
        if_space_released(self);
    }
    fn get_object_under_mouse(&self) -> Option<MP<Object>> {
        let mouse = self.get_mouse_world();
        for o in self.objects.iter() {
            if o.borrow().collider.point_inside(&mouse) {
                return Some(o.clone());
            }
        }

        return None;
    }
    fn handle_selected_object(&mut self) {
        let move_speed = 40.;
        let rot_speed = 1.;
        if (0..self.objects.len()).contains(&self.selected_obejct) {
            let o = self.objects.get(self.selected_obejct);
            let o = o.unwrap();
            use macroquad::input::KeyCode::{Down, Left, Right, Space, Up, A, D, S, W};
            let move_o = |v: V2| {
                o.borrow_mut()
                    .info
                    .physic
                    .kill_momentum();
                o.borrow_mut().info.physic.pos += v * self
                    .engine_time
                    .time_last_frame
                    .as_secs_f64();
            };
            let rot_o = |a: f64| {
                o.borrow_mut()
                    .info
                    .physic
                    .kill_momentum();
                o.borrow_mut().info.physic.ang += a * self
                    .engine_time
                    .time_last_frame
                    .as_secs_f64();
            };
            for k in macroquad::prelude::get_keys_down() {
                match k {
                    W => {
                        move_o(V2::new(
                            0.,
                            -move_speed,
                        ));
                    }
                    A => {
                        move_o(V2::new(
                            -move_speed,
                            -0.,
                        ));
                    }
                    S => {
                        move_o(V2::new(
                            0., move_speed,
                        ));
                    }
                    D => {
                        move_o(V2::new(
                            move_speed, 0.,
                        ));
                    }
                    Up => {}
                    Down => {}
                    Right => rot_o(rot_speed),
                    Left => rot_o(-rot_speed),
                    _ => (),
                }
            }

            for k in macroquad::prelude::get_keys_pressed() {
                match k {
                    Space => {
                        self.selected_obejct = (self.selected_obejct + 1) % self.objects.len();
                    }
                    _ => (),
                }
            }
        }
    }

    fn get_mouse_world(&self) -> V2 {
        let mouse = macroquad::input::mouse_position()
            .into_v2()
            .screen_to_world();

        return mouse;
    }

    fn draw(&mut self) {
        macroquad::prelude::clear_background(macroquad::color::BEIGE);
        for object in self.objects.iter() {
            object.borrow_mut().draw();
        }
        self.ui.draw();
        self.engine_physics_info
            ._collisions
            .draw_pairs();
        DEBBUGER.draw();
    }
}

fn if_space_pressed(engine: &mut Engine) {
    if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Space) {
        let object = engine.get_object_under_mouse();

        let new_widget_id = engine.ui.widjets_info.new_widget_id();
        if let Some(object) = object {
            engine.ui.add_widget(std::rc::Rc::new(
                RefCell::new(ImpulseAdder::new(
                    object,
                    engine.get_mouse_world(),
                    new_widget_id,
                )),
            ));
        }

        engine
            .ui
            .set_selected_widget(new_widget_id);
    }
}

#[rustfmt::skip]
fn if_space_held(engine: &mut Engine) {
    if macroquad::input::is_key_down(macroquad::input::KeyCode::Space) {
        engine.ui.press_selected_widget(Rc::new(
            ImpulseAdderInfo {
            mouse: macroquad::input::mouse_position()
                .into_v2()
                .screen_to_world(),
        }));
    }
    // todo!();
}
fn if_space_released(engine: &mut Engine) {
    if macroquad::input::is_key_released(macroquad::input::KeyCode::Space) {
        engine
            .ui
            .release_selected_widget(Rc::new(
                ImpulseAdderInfo {
                    mouse: macroquad::input::mouse_position()
                        .into_v2()
                        .screen_to_world(),
                },
            ));
    }
}

use crate::engine::ui::widjets::ImpulseAdderInfo;
use std::{cell::RefCell, rc::Rc};

use macroquad::{self};
mod graphics;
use graphics::ui;

mod helper_functions;

mod debugger;

use debugger::Debg;
use debugger::DEBBUGER;

mod objects;
use objects::{Object, MP, V2};

pub mod physics;
pub use physics::collisions;

mod engine_time;
use engine_time::EngineTime;

mod engine_physics_info;
use engine_physics_info::EnginePhysicsInfo;

mod engine_camera;
use engine_camera::EngineCamera;

use crate::engine::engine_camera::Conversionf32f32;

use self::{engine_camera::ConversionV2, graphics::ui::widjets::impulse_adder::ImpulseAdder};
