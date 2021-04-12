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

pub trait Intersectable {
    fn intersect<'a >(&'a self, world_ray: &Ray) -> Option<Box<dyn Intersection + 'a>>;
    fn bounds(&self) -> &AABB;
    fn entity_id(&self) -> u32;
    fn transform(&self) -> &Transform;
}

pub trait Renderable {
    fn is_emissive(&self) -> bool;
    // fn get_random_emissive_surface(&self, rng: &mut StdRng) -> Box<dyn Intersection + '_>;
}

pub trait SceneEntity : Intersectable + Renderable {}
/*
pub struct SceneEntity {
    pub model_instance: ModelInstance,
    pub inverse_transform: glm::Mat4,
    pub bounds: AABB,
    position: glm::Vec3,
    rotation: glm::Vec3,
    scale: glm::Vec3,
}

impl SceneEntity {
    pub fn new(model: ModelInstance) -> Self {
        // let inversed_world_transform = glm::inverse(&world_transform);
        // let transformed_bounds = mesh.bounds().transform(&world_transform);
        let bounds = model.bounds().clone();

        SceneEntity {
            model_instance: model,
            inverse_transform: glm::Matrix4::<f32>::one(),
            bounds, // TODO: FIX THIS LATER,
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
        let identity = glm::Matrix4::<f32>::one();
        let translation = glm::ext::translate(&identity, self.position);
        let scale = glm::ext::scale(&translation, self.scale);

        let rotation = glm::ext::rotate(&
                                            glm::ext::rotate(
                                                &glm::ext::rotate(
                                                    &scale,
                                                    self.rotation.x,
                                                    glm::vec3(1.0, 0.0, 0.0)),
                                                self.rotation.y,
                                                glm::vec3(0.0, 1.0, 0.0)),
                                        self.rotation.z,
                                        glm::vec3(0.0, 0.0, 1.0));

        self.bounds = self.model_instance.bounds().transform(&rotation);
        self.inverse_transform = glm::inverse(&rotation);
    }

    pub fn get_random_emissive_surface(&self, rng: &mut StdRng) -> u32 {
        /*for mesh in self.model_instance.model.meshes {
            // if self.model_instance.model.materials[mesh.material_index]
        }*/

        0
    }
}
*/
pub trait Scene {
    fn find_intersection(&self, ray: &crate::core::Ray) -> Option<Box<dyn Intersection + '_>>;
    // fn get_random_emissive_surface(&self, rng: &mut StdRng) -> Box<dyn Intersection + '_>;
    fn get_emissive_entities(&self) -> Vec<&Box<dyn SceneEntity+Sync+Send>>;
}