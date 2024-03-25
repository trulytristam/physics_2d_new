use macroquad::{self};
mod graphics;

mod helper_functions;

mod debug;
mod ui;

mod objects;
use objects::{Object, MP, V2};

mod physics;
use physics::collisions::CollisionDetectionAlgo;

mod engine_time;
use engine_time::EngineTime;

mod engine_physics_info;
use engine_physics_info::EnginePhysicsInfo;

mod engine_camera;
use engine_camera::EngineCamera;

use crate::engine::engine_camera::Conversionf32f32;

use self::engine_camera::ConversionV2;

pub struct Engine {
    objects: Vec<MP<Object>>,
    collision_detection_type: CollisionDetectionAlgo,
    engine_time: EngineTime,
    engine_physics_info: EnginePhysicsInfo,
    ui: ui::Ui,
    debugger: debug::Debugger,

    camera: EngineCamera,
}

impl Engine {
    pub fn new() -> Self {
        let temp = Engine::default();
        temp
    }

    pub fn default() -> Self {
        Engine {
            objects: vec![Object::new_poly_from_vec(vec![
                V2::new(-1., 1.),
                V2::new(0., 2.),
                V2::new(1., 1.),
                V2::new(0., -2.),
            ])
            .borrow()
            .translated(V2::new(0., 0.), 1.)],
            collision_detection_type: CollisionDetectionAlgo::GJK,
            engine_time: EngineTime::default(),
            engine_physics_info: EnginePhysicsInfo::default(),
            camera: EngineCamera::default(),
            ui: ui::Ui::default(),
            debugger: debug::Debugger::default(),
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
        for object in self.objects.iter_mut() {
            object
                .borrow_mut()
                .update(self.engine_time.clone(), self.engine_physics_info.clone());
        }
        self.get_object_under_mouse();
    }

    fn user_input(&mut self) {
        if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Space) {
            // let temp = ui::widjets::impulse_adder::ImpulseAdder::new(

            // )
        }
    }
    fn get_object_under_mouse(&self) -> Option<MP<Object>> {
        for o in self.objects.iter() {
            let mouse = macroquad::input::mouse_position()
                .into_v2()
                .screen_to_world(&self.camera);
            if o.borrow().collider.point_inside(&mouse) {
                self.debugger.draw_red();
                return Some(o.clone());
            }
        }

        return None;
    }

    fn draw(&mut self) {
        macroquad::prelude::clear_background(macroquad::color::BLACK);
        for object in self.objects.iter() {
            object.borrow_mut().draw(self.camera.clone());
        }
        self.debugger.draw();
    }
}
