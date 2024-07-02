use crate::{
    components::transform::Transform,
    constants::COLLISION_RANGE,
    models::{cube::Cube, plane::Plane},
    ray::{Intersection, Ray},
    utils::{create_transform_matrix, point_in_triangle},
};
use itertools::Itertools;
use nalgebra_glm::{Vec3, Vec4};

type HitboxVertices = (Vec3, Vec3);

pub enum HitboxKind {
    Cube,
    Plane,
    Sphere,
}

pub struct Hitbox {
    kind: HitboxKind,
    relative_transform: Transform,
}

pub struct Collider {
    plane_hitbox: Vec<HitboxVertices>,
    cube_hitbox: Vec<HitboxVertices>,
}

impl Collider {
    pub fn new() -> Self {
        let plane_hitbox = Plane::get_indexed_vertices();
        let cube_hitbox = Cube::get_indexed_vertices();

        Self {
            plane_hitbox,
            cube_hitbox,
        }
    }

    pub fn collides(&self, ray: &Ray, hitbox: &Hitbox, transform: &Transform) -> bool {
        let absolute_transform = create_transform_matrix(&transform);
        let relative_transform = create_transform_matrix(&hitbox.relative_transform);
        let transform = absolute_transform * relative_transform;
        let vertices = match hitbox.kind {
            HitboxKind::Plane => &self.plane_hitbox,
            HitboxKind::Cube => &self.cube_hitbox,
            HitboxKind::Sphere => return false,
        };
        for mut vertex in &vertices.iter().chunks(3) {
            let (a_position, a_normal) = vertex
                .next()
                .expect("Missing required vertex in collision mesh");
            let (b_position, _) = vertex
                .next()
                .expect("Missing required vertex in collision mesh");
            let (c_position, _) = vertex
                .next()
                .expect("Missing required vertex in collision mesh");

            let plane_normal = a_normal;

            let a_position = transform * Vec4::new(a_position.x, a_position.y, a_position.z, 1.0);

            let b_position = transform * Vec4::new(b_position.x, b_position.y, b_position.z, 1.0);

            let c_position = transform * Vec4::new(c_position.x, c_position.y, c_position.z, 1.0);

            if let Some(Intersection { distance, point }) = ray.intersects(&plane_normal) {
                let triangle = (a_position.xyz(), b_position.xyz(), c_position.xyz());

                if distance <= COLLISION_RANGE
                    && distance >= 0.0
                    && point_in_triangle(point, triangle)
                {
                    return true;
                }
            }
        }

        false
    }
}
