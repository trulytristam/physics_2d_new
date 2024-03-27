use super::objects::V2;

pub fn gen_perp_matrix() -> nalgebra::Matrix2<f64> {
    nalgebra::Matrix2::new(0., -1., 1., 0.)
}

///Shape should have clockwise winding
pub fn point_inside_shape(shape: &Vec<V2>, point: &V2) -> bool {
    for i in 0..shape.len() {
        let a = shape[i];
        let b = shape[(i + 1) % shape.len()];
        let v = b - a;
        let o = point - a;
        let n = gen_perp_matrix() * v;

        if o.dot(&n) > 0. {
            return false;
        }
    }
    return true;
}
