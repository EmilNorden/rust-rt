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
        let mut transform = glm::ext::scale(&glm::Mat4::one(), self.scale);
        transform = glm::ext::rotate(&transform, self.rotation.z, glm::vec3(0.0, 0.0, 1.0));
        transform = glm::ext::rotate(&transform, self.rotation.y, glm::vec3(0.0, 1.0, 0.0));
        transform = glm::ext::rotate(&transform, self.rotation.x, glm::vec3(1.0, 0.0, 0.0));
        self.world = glm::ext::translate(&transform, self.translation);
        self.inverse_world = glm::inverse(&self.world);
    }
}