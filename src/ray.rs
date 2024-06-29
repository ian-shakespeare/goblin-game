use nalgebra_glm as glm;

use crate::components::collision::CollisionComponent;

pub struct Ray {
    origin: glm::Vec3,
    direction: glm::Vec3,
}

impl Ray {
    pub fn new(origin: glm::Vec3, direction: glm::Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn intersects(&self, plane: (&glm::Vec3, &glm::Vec3)) -> Option<f32> {
        let (position, normal) = plane;
        let denominator = normal.dot(&self.direction);
        if denominator == 0.0 {
            None
        } else {
            let distance_to_intersection = (position - self.origin).dot(&normal) / denominator;
            Some(distance_to_intersection)
        }
    }

    pub fn collides(&self, plane: &CollisionComponent, max_collision_distance: f32) -> bool {
        let [a, b, c] = plane.vertices;

        if let Some(distance_to_intersection) =
            self.intersects((&glm::Vec3::new(0.0, 0.0, 0.0), &plane.normal))
        {
            let plane_a_normal = (c - a).cross(&(c - plane.normal));
            let plane_a_intersection = self.intersects((&a, &plane_a_normal));
            if let Some(distance) = plane_a_intersection {
                if distance < distance_to_intersection && distance >= 0.0 {
                    return false;
                }
            }

            let plane_b_normal = (b - a).cross(&(b - plane.normal));
            let plane_b_intersection = self.intersects((&b, &plane_b_normal));
            if let Some(distance) = plane_b_intersection {
                if distance < distance_to_intersection && distance >= 0.0 {
                    return false;
                }
            }

            let plane_c_normal = (b - c).cross(&(c - plane.normal));
            let plane_c_intersection = self.intersects((&c, &plane_c_normal));
            if let Some(distance) = plane_c_intersection {
                if distance < distance_to_intersection && distance >= 0.0 {
                    return false;
                }
            }

            return distance_to_intersection <= max_collision_distance
                && distance_to_intersection >= 0.0;
        }

        false
    }
}
