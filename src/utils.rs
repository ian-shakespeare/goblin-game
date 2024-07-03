use nalgebra_glm::{self as glm, Mat4, Vec3};
use std::f32::consts::PI;

use crate::components::transform::Transform;

pub fn create_empty_buffer(len: usize) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    buffer
}

pub fn degree_to_radian(degree: f32) -> f32 {
    degree * (PI / 180.0)
}

pub fn point_in_triangle(point: Vec3, triangle: (Vec3, Vec3, Vec3)) -> bool {
    let (mut a, mut b, mut c) = triangle;

    a -= point;
    b -= point;
    c -= point;

    let u = b.cross(&c);
    let v = c.cross(&a);
    let w = a.cross(&b);

    if u.dot(&v) < 0.0 {
        return false;
    }

    if u.dot(&w) < 0.0 {
        return false;
    }

    true
}

pub fn create_transform_matrix(transform: &Transform) -> Mat4 {
    let position = transform.position();
    let rotation = transform.rotation();
    let scale = transform.scale();

    let mut transform = Mat4::identity();
    transform = glm::translate(&transform, &position);
    if let Some(rotation) = rotation {
        let angle = degree_to_radian(rotation.x);
        let axis = Vec3::new(rotation.y, rotation.z, rotation.w);
        transform = glm::rotate(&transform, angle, &axis);
    }
    if let Some(scale) = scale {
        transform = glm::scale(&transform, &scale);
    }

    transform
}

pub fn heighten_vector(vector: Vec3, height: f32) -> Vec3 {
    Vec3::new(vector.x, vector.y + height, vector.z)
}

pub fn lengthen_vector(vector: Vec3, length: f32) -> Vec3 {
    Vec3::new(vector.x + length, vector.y, vector.z)
}

pub fn widen_vector(vector: Vec3, width: f32) -> Vec3 {
    Vec3::new(vector.x, vector.y, vector.z + width)
}

pub fn flatten_vector(vector: Vec3) -> Vec3 {
    Vec3::new(vector.x, 0.0, vector.z)
}

pub fn tuple_to_vec(tuple: (f32, f32, f32)) -> Vec3 {
    let (x, y, z) = tuple;

    Vec3::new(x, y, z)
}
