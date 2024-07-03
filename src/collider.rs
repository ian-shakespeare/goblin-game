use crate::{
    components::transform::Transform,
    constants::COLLISION_RANGE,
    models::plane::Plane,
    ray::{Intersection, Ray},
    utils::{create_transform_matrix, point_in_triangle},
};
use itertools::Itertools;
use nalgebra_glm::{self as glm, Mat4, Vec3, Vec4};

pub struct Collider {
    hit_plane: Vec<(Vec3, Vec3)>,
    collidables: Vec<Transform>,
}

impl Collider {
    pub fn new() -> Self {
        let hit_plane = Plane::get_indexed_vertices();

        Self {
            collidables: Vec::new(),
            hit_plane,
        }
    }

    pub fn add_collidable(&mut self, transform: Transform) {
        self.collidables.push(transform);
    }

    fn get_transformed_vertices(&self, transform: &Transform) -> Vec<(Vec3, Vec3)> {
        let rotation = match transform.rotation() {
            None => Mat4::identity(),
            Some(rotation) => {
                glm::rotation(rotation.x, &Vec3::new(rotation.y, rotation.z, rotation.w))
            }
        };
        let transform = create_transform_matrix(transform);
        self.hit_plane
            .iter()
            .map(|(position, normal)| {
                let position = Vec4::new(position.x, position.y, position.z, 1.0);
                let transformed_position = transform * position;
                let normal = Vec4::new(normal.x, normal.y, normal.z, 1.0);
                let transformed_normal = rotation * normal;

                (transformed_position.xyz(), transformed_normal.xyz())
            })
            .collect()
    }

    pub fn collides(&self, ray: &Ray) -> bool {
        for collidable in self.collidables.iter() {
            let vertices = self.get_transformed_vertices(&collidable);

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

                if let Some(Intersection { distance, point }) = ray.intersects(&plane_normal) {
                    let triangle = (a_position.clone(), b_position.clone(), c_position.clone());

                    if distance <= COLLISION_RANGE
                        && distance >= 0.0
                        && point_in_triangle(point, triangle)
                    {
                        return true;
                    }
                }
            }
        }

        false
    }
}
