pub mod geom;

pub struct Intersection {
    pub u: f32,
    pub v: f32,
    pub distance: f32
}

#[allow(dead_code)]
pub struct Ray {
    pub origin: glm::Vector3<f32>,
    pub direction: glm::Vector3<f32>,
}


impl Ray {
    pub fn transform(&self, matrix: &glm::Mat4) -> Ray {
        let new_origin = *matrix * glm::Vector4::new(self.origin.x, self.origin.y, self.origin.z, 1.0);
        let new_direction = glm::normalize(*matrix * glm::Vector4::new(self.direction.x, self.direction.y, self.direction.z, 0.0));
        Ray {
            origin: glm::Vector3::new(new_origin.x, new_origin.y, new_origin.z),
            direction: glm::Vector3::new(new_direction.x, new_direction.y, new_direction.z),
        }
    }
}