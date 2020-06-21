use crate::core::Intersection;
use crate::scene::naive_scene::NaiveScene;
use crate::content::mesh::IndexedMesh;

mod naive_scene;
pub mod octree_scene;

pub struct SceneEntity<'a> {
    pub mesh: &'a IndexedMesh,
    pub inverse_transform: glm::Mat4,
    // pub bounds: AABB,
}

impl<'a> SceneEntity<'a> {
    pub fn new(mesh: &'a IndexedMesh, inverse_transform: glm::Mat4) -> SceneEntity<'a> {
        // let transformed_bounds = mesh.bounds.transform(&transform);

        SceneEntity {
            mesh,
            inverse_transform
        }
    }
}

pub trait Scene<'a> {
    fn trace(&self, ray: &crate::core::Ray) -> Option<Intersection>;
    fn add(&mut self, entity: SceneEntity<'a>);
}

pub fn create_scene<'a>() -> impl Scene<'a> {
        return NaiveScene::new()
}
