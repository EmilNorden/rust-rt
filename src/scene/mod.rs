use crate::core::{Intersection, Ray};
use crate::core::geom::AABB;
use crate::content::octree_mesh::OctreeMesh;
use crate::content::model::{Model, ModelInstance};
use num_traits::One;
use std::rc::Rc;
use glm::ext::rotate;
use crate::content::material::Material;
use rand::rngs::StdRng;
use crate::scene::transform::Transform;

pub mod octree_scene;
pub mod sphere_entity;
pub mod mesh_entity;
pub mod transform;
pub mod transform_builder;
pub mod plane_entity;

pub trait Intersectable {
    fn intersect<'a >(&'a self, world_ray: &Ray) -> Option<Box<dyn Intersection + 'a>>;
    fn bounds(&self) -> Option<&AABB>;
    fn entity_id(&self) -> u32;
    fn transform(&self) -> &Transform;
}

pub struct SurfaceDescription {
    pub coordinate: glm::Vec3,
    pub world_normal: glm::Vec3,
    pub emission: glm::Vec3,
    pub entity_id: u32,
}

pub trait Renderable {
    fn is_emissive(&self) -> bool;
    fn get_random_emissive_surface(&self, rng: &mut StdRng) -> SurfaceDescription;
    // fn get_random_emissive_surface(&self, rng: &mut StdRng) -> Box<dyn Intersection + '_>;
}

pub trait SceneEntity : Intersectable + Renderable {}

pub trait Scene {
    fn find_intersection(&self, ray: &crate::core::Ray) -> Option<Box<dyn Intersection + '_>>;
    // fn get_random_emissive_surface(&self, rng: &mut StdRng) -> Box<dyn Intersection + '_>;
    fn get_emissive_entities(&self) -> Vec<&Box<dyn SceneEntity+Sync+Send>>;
}