use macroquad::prelude::*;
mod graphics;

mod objects;
use objects::Object;

mod physics;
use physics::collisions::CollisionDetectionAlgo;

mod engine_time;
use engine_time::EngineTime;

mod engine_physics_info;
use engine_physics_info::EnginePhysicsInfo;

mod engine_camera;
use engine_camera::EngineCamera;

use std::cell::RefCell;
use std::rc::Rc;

type V2 = nalgebra::Vector2<f64>;

pub struct Engine {
    objects: Vec<Rc<RefCell<Object>>>,
    collision_detection_type: CollisionDetectionAlgo,
    engine_time: EngineTime,
    engine_physics_info: EnginePhysicsInfo,

    camera: EngineCamera,
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
                .borrow()
                .translated(V2::new(-120., 0.), 1.),
                Object::new_poly_from_vec(vec![
                    V2::new(-1., 1.),
                    V2::new(0., 2.),
                    V2::new(1., 1.),
                    V2::new(0., -2.),
                ])
                .borrow()
                .translated(V2::new(120., 0.), 4.),
            ],
            collision_detection_type: CollisionDetectionAlgo::GJK,
            engine_time: EngineTime::default(),
            engine_physics_info: EnginePhysicsInfo::default(),
            camera: EngineCamera::default(),
        }
    }

    pub async fn start(&mut self) {
        loop {
            self.update();
            self.draw();
            next_frame().await
        }
    }

    fn update(&mut self) {
        self.engine_time.frame_start();
        for object in self.objects.iter_mut() {
            object
                .borrow_mut()
                .update(self.engine_time.clone(), self.engine_physics_info.clone());
        }
    }

    fn draw(&mut self) {
        clear_background(BLACK);
        for object in self.objects.iter() {
            object.borrow_mut().draw(self.camera.clone());
        }
    }
}
