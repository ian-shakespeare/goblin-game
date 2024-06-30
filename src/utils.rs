use std::f32::consts::PI;
use nalgebra_glm::Vec3;

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
