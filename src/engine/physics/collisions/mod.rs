mod gjk;

use crate::engine::objects::colliders::Manifold;
pub mod helper_functions;

use super::super::Object;
use std::cell::RefCell;
use std::rc::Rc;

type MP<T> = Rc<RefCell<T>>;

pub enum CollisionDetectionAlgo {
    GJK,
}

struct CollisionPair {
    a: MP<Object>,
    b: MP<Object>,
}

struct EngineCollisionInfo {
    pairs: Vec<Manifold>,
}

impl EngineCollisionInfo {
    fn generate_pairs(&mut self, objects: &Vec<MP<Object>>) {
        self.pairs.clear();

        if objects.len() < 2 {
            return;
        }

        for a in 0..(objects.len() - 1) {
            let object_a = &objects[a];
            for b in a..objects.len() {
                let object_b = &objects[b];
                let manifold: Option<Manifold> = object_a
                    .borrow_mut()
                    .collider
                    .is_colliding(&object_b.borrow_mut().collider);
                if let Some(m) = manifold {
                    self.pairs.push(m);
                }
            }
        }
    }
}
