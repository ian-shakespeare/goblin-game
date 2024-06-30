use crate::{components::transform::TransformComponent, mesh::Mesh, utils::point_in_triangle};
use itertools::Itertools;
use nalgebra_glm::{self as glm, Mat4, Vec3, Vec4};

pub struct Intersection {
    distance: f32,
    point: Vec3,
}

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn intersects(&self, plane_normal: &Vec3) -> Option<Intersection> {
        let denominator = self.direction.dot(plane_normal);
        if denominator == 0.0 {
            None
        } else {
            let distance = -(self.origin.dot(plane_normal)) / denominator;
            let point = self.origin + distance * self.direction;
            Some(Intersection { distance, point })
        }
    }

    pub fn collides(
        &self,
        mesh: &Mesh,
        mesh_transform: TransformComponent,
        max_collision_distance: f32,
    ) -> bool {
        for mut vertex in &mesh.indexed_vertices().chunks(3) {
            let a = vertex
                .next()
                .expect("Missing required vertex in collision mesh");
            let b = vertex
                .next()
                .expect("Missing required vertex in collision mesh");
            let c = vertex
                .next()
                .expect("Missing required vertex in collision mesh");

            let plane_normal = a.normal;

            let mut transform = Mat4::identity();
            transform = glm::translate(&transform, &mesh_transform.position);
            if let Some(rotation) = mesh_transform.rotation {
                transform = glm::rotate(
                    &transform,
                    rotation[0],
                    &Vec3::new(rotation[1], rotation[2], rotation[3]),
                );
            }
            if let Some(scale) = mesh_transform.scale {
                transform = glm::scale(&transform, &scale);
            }

            let a_position = a.position;
            let a_position = transform * Vec4::new(a_position.x, a_position.y, a_position.z, 1.0);

            let b_position = b.position;
            let b_position = transform * Vec4::new(b_position.x, b_position.y, b_position.z, 1.0);

            let c_position = c.position;
            let c_position = transform * Vec4::new(c_position.x, c_position.y, c_position.z, 1.0);

            if let Some(Intersection { distance, point }) = self.intersects(&plane_normal) {
                let triangle = (a_position.xyz(), b_position.xyz(), c_position.xyz());

                if distance <= max_collision_distance
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
