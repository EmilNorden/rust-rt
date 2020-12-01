use crate::core::Intersection;
use crate::core::geom::AABB;
use crate::content::octree_mesh::OctreeMesh;
use crate::content::model::Model;
use num_traits::One;

mod naive_scene;
pub mod octree_scene;

pub struct SceneEntity<'a> {
    pub mesh: &'a Model,
    pub inverse_transform: glm::Mat4,
    pub bounds: AABB,
    position: glm::Vec3,
    rotation: glm::Vec3,
    scale: glm::Vec3,
}

impl<'a> SceneEntity<'a> {
    pub fn new(mesh: &'a Model) -> SceneEntity<'a> {
        // let inversed_world_transform = glm::inverse(&world_transform);
        // let transformed_bounds = mesh.bounds().transform(&world_transform);


        SceneEntity {
            mesh,
            inverse_transform:  glm::Matrix4::<f32>::one(),
            bounds: mesh.bounds.clone(), // TODO: FIX THIS LATER,
            position: glm::Vec3::new(0.0, 0.0, 0.0),
            rotation: glm::Vec3::new(0.0, 0.0, 0.0),
            scale: glm::Vec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn set_position(&mut self, value: glm::Vec3) -> &Self {
        self.position = value;
        self.update_transform();
        self
    }

    pub fn set_rotation(&mut self, value: glm::Vec3) -> &Self {
        self.rotation = value;
        self.update_transform();
        self
    }

    pub fn set_scale(&mut self, value: glm::Vec3) -> &Self {
        self.scale = value;
        self.update_transform();
        self
    }

    fn update_transform(&mut self) {

    }
}

pub trait Scene<'a> {
    fn trace(&self, ray: &crate::core::Ray) -> Option<Intersection>;
    fn add(&mut self, entity: SceneEntity<'a>);
}