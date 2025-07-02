mod transform;
mod vectors;

pub use glam::f32::*;
use three_d::{Matrix4, Vector3};
pub use transform::*;
pub use vectors::*;

pub fn to_glam_vec(vec: Vector3<f32>) -> Vec3 {
    Vec3::new(vec.x, vec.y, vec.z)
}

pub fn from_glam_vec(vec: Vec3) -> Vector3<f32> {
    Vector3::new(vec.x, vec.y, vec.z)
}

pub fn from_glam_mat4(mat: Mat4) -> Matrix4<f32> {
    let cols = &mat.to_cols_array_2d();
    Matrix4::from_cols(cols[0].into(), cols[1].into(), cols[2].into(), cols[3].into(),)
}