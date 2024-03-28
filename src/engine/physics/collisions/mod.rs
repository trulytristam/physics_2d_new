pub mod gjk;

use macroquad::experimental::camera::mouse::Camera;
use macroquad::miniquad::RenderingBackend;

use crate::engine::debugger::Debg;
use crate::engine::engine_camera::{ConversionV2, EngineCamera};
use crate::engine::objects::colliders::Manifold;
use crate::engine::objects::{MP, V2};
pub mod helper_functions;

use self::gjk::gjk;

use super::super::Object;
use crate::engine::DEBBUGER;

pub enum CollisionDetectionAlgo {
    GJK,
}

#[derive(Default, Clone)]
pub struct EngineCollisionInfo {
    pairs: Vec<Manifold>,
}

impl EngineCollisionInfo {
    pub fn generate_pairs(&mut self, objects: &Vec<MP<Object>>) {
        self.pairs.clear();
        if objects.len() < 2 {
            return;
        }
        for a in 0..(objects.len() - 1) {
            let object_a = &objects[a];
            for b in (a + 1)..objects.len() {
                let object_b = &objects[b];

                let gjk_result = gjk(
                    &object_a.clone().borrow().collider,
                    &object_b.clone().borrow().collider,
                );
                if gjk_result.is_colliding {
                    let m = Manifold {
                        a: object_a.clone(),
                        b: object_b.clone(),
                        collision_normal: V2::new(0., 0.),
                        collision_point: V2::new(0., 0.),
                    };
                    self.pairs.push(m);
                }
            }
        }
    }

    pub fn draw_pairs(&self, cam: &EngineCamera) {
        for pair in self.pairs.iter() {
            let a = pair.a.borrow().info.physic.pos.world_to_screen(cam);
            let b = pair.a.borrow().info.physic.pos.world_to_screen(cam);
            DEBBUGER.draw_arrow(a, b, macroquad::prelude::RED);
        }
    }
    pub fn count_pairs(&self) -> usize {
        return self.pairs.len();
    }
}
