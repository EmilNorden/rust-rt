use crate::scene::{Intersectable, Renderable, SceneEntity};
use crate::core::{Intersection, Ray};
use crate::core::geom::AABB;
use glm::{Vec2, Vec3};
use crate::content::material::Material;
use num_traits::{Zero, One};
use rand::rngs::StdRng;
use rand::Rng;

pub struct SphereIntersection<'a> {
    entity_id: u32,
    material: &'a Material,
    transform: &'a glm::Mat4,
    hit_point: glm::Vec3,
    sphere_position: glm::Vec3,
    distance: f32,
}

impl Intersection for SphereIntersection<'_> {
    fn coordinate(&self) -> Vec3 {
        let tmp = *self.transform * glm::vec4(self.hit_point.x, self.hit_point.y, self.hit_point.z, 1.0);
        glm::vec3(tmp.x, tmp.y, tmp.z)
    }

    fn object_space_normal(&self) -> glm::Vec4 {
        let tmp = glm::normalize(self.hit_point);
        glm::vec4(tmp.x, tmp.y, tmp.z, 0.0)
    }

    fn world_space_normal(&self) -> Vec3 {
        let tmp = *self.transform * self.object_space_normal();
        glm::vec3(tmp.x, tmp.y, tmp.z)
    }

    fn texture_coordinates(&self) -> Vec2 {
        glm::vec2(0.0, 0.0)
    }

    fn material(&self) -> &Material {
        self.material
    }

    fn distance(&self) -> f32 {
        self.distance
    }

    fn entity_id(&self) -> u32 {
        self.entity_id
    }

    fn is_same_surface(&self, other: Box<dyn Intersection>) -> bool {
        if self.entity_id() != other.entity_id() {
            return false;
        }

        if glm::ext::sqlength(self.coordinate() - other.coordinate()) > 0.1 {
            return false;
        }

        true
    }
}

pub struct SphereEntity {
    entity_id: u32,
    position: glm::Vec3,
    rotation: glm::Vec3,
    scale: glm::Vec3,
    radius: f32,
    bounds: AABB,
    material: Material,
    transform: glm::Mat4,
    inverse_transform: glm::Mat4,
}

impl SphereEntity {
    pub fn new(id: u32, position: glm::Vec3, rotation: glm::Vec3, scale: glm::Vec3, radius: f32, material: Material) -> Self {
        let transform = Self::build_transform(&position, &rotation, &scale);
        let inverse_transform = glm::inverse(&transform);
        let bounds = AABB {
            min: position - glm::vec3(radius, radius, radius),
            max: position + glm::vec3(radius, radius, radius),
        };


        SphereEntity {
            entity_id: id,
            position,
            rotation,
            scale,
            radius,
            bounds: bounds.transform(&transform),
            material,
            transform,
            inverse_transform,
        }
    }

    fn build_transform(position: &glm::Vec3, rotation: &glm::Vec3, scale: &glm::Vec3) -> glm::Mat4 {
        let mut transform = glm::ext::scale(&glm::Mat4::one(), *scale);
        transform = glm::ext::rotate(&transform, rotation.z, glm::vec3(0.0, 0.0, 1.0));
        transform = glm::ext::rotate(&transform, rotation.y, glm::vec3(0.0, 1.0, 0.0));
        transform = glm::ext::rotate(&transform, rotation.x, glm::vec3(1.0, 0.0, 0.0));
        glm::ext::translate(&transform, *position)
    }

    fn intersect_object_ray(&self, object_ray: &Ray) -> Option<Box<dyn Intersection + '_>> {
        // let oc = world_ray.origin - self.position;
        let oc = object_ray.origin - glm::vec3(0.0, 0.0, 0.0);
        let a = glm::dot(object_ray.direction, object_ray.direction);
        let b = 2.0 * glm::dot(oc, object_ray.direction);

        // If sphere is behind ray
        if b > 0.0 {
            return None;
        }

        let c = glm::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let distance = (-b - glm::sqrt(discriminant)) / (2.0 * a);
        if distance > 0.0 {
            let fff = 334;
        }
        Some(Box::new(SphereIntersection {
            entity_id: self.entity_id,
            material: &self.material,
            transform: &self.transform,
            hit_point: object_ray.origin + (object_ray.direction * distance),
            sphere_position: self.position,
            distance,
        }))
    }
}

impl SceneEntity for SphereEntity {}

impl Renderable for SphereEntity {
    fn is_emissive(&self) -> bool {
        !self.material.emission().is_zero()
    }

    fn get_random_emissive_surface(&self, rng: &mut StdRng) -> Box<dyn Intersection + '_> {
        let random_point = glm::normalize(glm::vec3(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            - glm::vec3(0.5, 0.5, 0.5)) * self.radius;

        Box::new(SphereIntersection {
            entity_id: self.entity_id,
            material: &self.material,
            transform: &self.transform,
            hit_point: random_point,
            sphere_position: self.position,
            distance: 0.0,
        })
    }
}

impl Intersectable for SphereEntity {
    fn intersect(&self, world_ray: &Ray) -> Option<Box<dyn Intersection + '_>> {
        let object_ray = world_ray.transform(&self.inverse_transform);
        self.intersect_object_ray(&object_ray)
    }

    fn bounds(&self) -> &AABB {
        &self.bounds
    }

    fn entity_id(&self) -> u32 {
        self.entity_id
    }

    fn position(&self) -> glm::Vec3 { self.position }
}