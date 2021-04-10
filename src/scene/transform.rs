use num_traits::One;

pub struct Transform {
    translation: glm::Vec3,
    rotation: glm::Vec3,
    scale: glm::Vec3,
    world: glm::Mat4,
    inverse_world: glm::Mat4,
}


impl Transform {
    pub fn new() -> Self {
        Transform {
            translation: glm::vec3(0.0, 0.0, 0.0),
            rotation: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
            world: glm::Mat4::one(),
            inverse_world: glm::Mat4::one(),
        }
    }
    pub fn translation(&self) -> &glm::Vec3 { &self.translation }
    pub fn rotation(&self) -> &glm::Vec3 { &self.rotation }
    pub fn scale(&self) -> &glm::Vec3 { &self.scale }

    pub fn set_translation(&mut self, value: glm::Vec3) {
        self.translation = value;
        self.build_transform();
    }

    pub fn set_rotation(&mut self, value: glm::Vec3) {
        self.rotation = value;
        self.build_transform();
    }

    pub fn set_scale(&mut self, value: glm::Vec3) {
        self.scale = value;
        self.build_transform();
    }

    pub fn world(&self) -> &glm::Mat4 {
        &self.world
    }

    pub fn inverse_world(&self) -> &glm::Mat4 {
        &self.inverse_world
    }

    fn build_transform(&mut self) {
        let mut transform = glm::ext::translate(&glm::Mat4::one(), self.translation);
        transform = glm::ext::rotate(&transform, self.rotation.z, glm::vec3(0.0, 0.0, 1.0));
        transform = glm::ext::rotate(&transform, self.rotation.y, glm::vec3(0.0, 1.0, 0.0));
        transform = glm::ext::rotate(&transform, self.rotation.x, glm::vec3(1.0, 0.0, 0.0));
        self.world = glm::ext::scale(&transform, self.scale);
        self.inverse_world = glm::inverse(&self.world);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_translation_should_create_valid_world() {
        let mut transform = Transform::new();
        transform.set_translation(glm::vec3(100.0, 200.0, 300.0));

        let result = transform.world();
        assert_eq!(result.c0, glm::vec4(1.0, 0.0, 0.0, 0.0));
        assert_eq!(result.c1, glm::vec4(0.0, 1.0, 0.0, 0.0));
        assert_eq!(result.c2, glm::vec4(0.0, 0.0, 1.0, 0.0));
        assert_eq!(result.c3, glm::vec4(100.0, 200.0, 300.0, 1.0));
    }

    #[test]
    fn set_scale_should_create_valid_world() {
        let mut transform = Transform::new();
        transform.set_scale(glm::vec3(100.0, 200.0, 300.0));

        let result = transform.world();
        assert_eq!(result.c0, glm::vec4(100.0, 0.0, 0.0, 0.0));
        assert_eq!(result.c1, glm::vec4(0.0, 200.0, 0.0, 0.0));
        assert_eq!(result.c2, glm::vec4(0.0, 0.0, 300.0, 0.0));
        assert_eq!(result.c3, glm::vec4(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn composed_transform_should_create_valid_world() {
        let mut transform = Transform::new();
        transform.set_translation(glm::vec3(100.0, 200.0, 300.0));
        transform.set_scale(glm::vec3(4.0, 3.0, 2.0));

        let result = transform.world();
        assert_eq!(result.c0, glm::vec4(4.0, 0.0, 0.0, 0.0));
        assert_eq!(result.c1, glm::vec4(0.0, 3.0, 0.0, 0.0));
        assert_eq!(result.c2, glm::vec4(0.0, 0.0, 2.0, 0.0));
        assert_eq!(result.c3, glm::vec4(100.0, 200.0, 300.0, 1.0));
    }

    #[test]
    fn new_transform_should_equal_identity() {
        let mut transform = Transform::new();

        let result = transform.world();
        assert_eq!(result.c0, glm::vec4(1.0, 0.0, 0.0, 0.0));
        assert_eq!(result.c1, glm::vec4(0.0, 1.0, 0.0, 0.0));
        assert_eq!(result.c2, glm::vec4(0.0, 0.0, 1.0, 0.0));
        assert_eq!(result.c3, glm::vec4(0.0, 0.0, 0.0, 1.0));
    }
}