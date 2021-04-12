use crate::content::material::{Texture, Material};

pub struct MaterialBuilder {
    diffuse_map: Option<Texture>,
    diffuse_color: Option<glm::Vec3>,
    emissive_color: Option<glm::Vec3>,
    reflectivity: f32,
    transparency: bool,
    refractive_index : f32,
}

impl MaterialBuilder {
    pub fn new() -> Self {
        MaterialBuilder {
            diffuse_map: None,
            diffuse_color: None,
            emissive_color: None,
            reflectivity: 0.0,
            transparency: false,
            refractive_index: 0.0
        }
    }

    pub fn with_diffuse_map(&mut self, texture: Texture) -> &mut MaterialBuilder {
        self.diffuse_map = Some(texture);
        self
    }

    pub fn with_diffuse_color(&mut self, color: glm::Vec3) -> &mut MaterialBuilder {
        self.diffuse_color = Some(color);
        self
    }

    pub fn with_emissive_color(&mut self, color: glm::Vec3) -> &mut MaterialBuilder {
        self.emissive_color = Some(color);
        self
    }

    pub fn with_reflectivity(&mut self, value: f32) -> &mut MaterialBuilder {
        self.reflectivity = value;
        self
    }

    pub fn with_transparency(&mut self, refractive_index: f32) -> &mut MaterialBuilder {
        self.transparency = true;
        self.refractive_index = refractive_index;
        self
    }

    pub fn build(&self) -> Material {
        Material::new(
            self.diffuse_map.clone(),
            self.diffuse_color.unwrap_or(glm::vec3(0.0, 0.0, 0.0)),
        self.emissive_color.unwrap_or(glm::vec3(0.0, 0.0, 0.0)),
        self.reflectivity,
        self.transparency,
        self.refractive_index)
    }
}

