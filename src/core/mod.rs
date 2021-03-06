use crate::content::octree_mesh::OctreeMesh;
use crate::content::material::Material;

pub mod geom;
pub mod math;
pub mod plane;

pub trait Intersection {
    fn coordinate(&self) -> glm::Vec3;
    fn world_space_normal(&self) -> glm::Vec3;
    fn texture_coordinates(&self) -> glm::Vec2;
    fn material(&self) -> &Material;
    fn distance(&self) -> f32;
    fn entity_id(&self) -> u32;
    fn is_same_surface(&self, other: Box<dyn Intersection>) -> bool;
}

/*pub struct Intersection<'a> {
    pub mesh: &'a OctreeMesh,
    pub material: Option<&'a Material>,
    pub u: f32,
    pub v: f32,
    pub indices: (u32, u32, u32),
    pub distance: f32,
    pub material_index: usize,
}*/

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