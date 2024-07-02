use nalgebra_glm::Vec3;

pub struct Intersection {
    pub distance: f32,
    pub point: Vec3,
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
}
