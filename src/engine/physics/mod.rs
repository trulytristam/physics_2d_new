use super::objects::colliders::Manifold;

pub mod collisions;

pub fn solve_constaint(mani: &Manifold) {
    let a = mani.a.borrow_mut();
    let b = mani.b.borrow_mut();

    let v_rel = b.get_linear_vel_at_point(&mani.collision_point)
        - a.get_linear_vel_at_point(&mani.collision_point);
}
