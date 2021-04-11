#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn black() -> Self {
        Color {
            r: 0,
            g: 0,
            b: 0,
        }
    }
    pub fn from_vec3(vec: &glm::Vec3) -> Self {
        Color {
            r: (vec.x * 255.0) as u8,
            g: (vec.y * 255.0) as u8,
            b: (vec.z * 255.0) as u8
        }
    }
}