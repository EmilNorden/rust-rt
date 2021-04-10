use crate::scene::transform::Transform;

pub struct TransformBuilder {
    translation: glm::Vec3,
    rotation: glm::Vec3,
    scale: glm::Vec3,
}

impl TransformBuilder {
    pub fn new() -> Self {
        TransformBuilder {
            translation: glm::vec3(0.0, 0.0, 0.0),
            rotation: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0)
        }
    }

    pub fn with_translation(&mut self, value: glm::Vec3) -> &mut TransformBuilder {
        self.translation = value;
        self
    }

    pub fn with_rotation(&mut self, value: glm::Vec3) -> &mut TransformBuilder {
        self.rotation = value;
        self
    }

    pub fn with_scale(&mut self, value: glm::Vec3) -> &mut TransformBuilder {
        self.scale = value;
        self
    }

    pub fn build(&mut self) -> Transform  {
        let mut transform = Transform::new();
        transform.set_translation(self.translation);
        transform.set_rotation(self.rotation);
        transform.set_scale(self.scale);

        transform

    }
}