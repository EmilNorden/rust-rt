use crate::core::Intersection;
use crate::core::geom::AABB;
use crate::content::octree_mesh::OctreeMesh;

mod naive_scene;
pub mod octree_scene;

pub struct SceneEntity<'a> {
    pub mesh: &'a OctreeMesh,
    pub inverse_transform: glm::Mat4,
    pub bounds: AABB,
}

impl<'a> SceneEntity<'a> {
    pub fn new(mesh: &'a OctreeMesh, world_transform: glm::Mat4) -> SceneEntity<'a> {
        let inversed_world_transform = glm::inverse(&world_transform);
        let transformed_bounds = mesh.bounds().transform(&world_transform);


        SceneEntity {
            mesh,
            inverse_transform: inversed_world_transform,
            bounds: transformed_bounds
        }
    }
}

pub trait Scene<'a> {
    fn trace(&self, ray: &crate::core::Ray) -> Option<Intersection>;
    fn add(&mut self, entity: SceneEntity<'a>);
}