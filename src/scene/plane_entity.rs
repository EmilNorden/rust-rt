use crate::core::geom::{AABB, ray_aabb_intersect};
use crate::content::material::Material;
use crate::scene::transform::Transform;
use float_cmp::{ApproxEq, F32Margin};
use crate::scene::{SceneEntity, Renderable, SurfaceDescription, Intersectable};
use rand::prelude::StdRng;
use crate::core::{Intersection, Ray};
use num_traits::Zero;
use crate::core::plane::Plane;
use rand::Rng;
use glm::{Vec2, Vec3, Vec4};

pub struct PlaneEntity {
    entity_id: u32,
    plane: Plane,
    extent: f32,
    bounds: AABB,
    material: Material,
    transform: Transform,
}

impl PlaneEntity {
    pub fn new(entity_id: u32, plane: Plane, extent: f32, material: Material, transform: Transform) -> Self {
        let bounds = AABB::from_plane(&plane, extent, 0.1);
        PlaneEntity {
            entity_id,
            plane,
            extent,
            material,
            transform,
            bounds,
        }
    }
}

impl SceneEntity for PlaneEntity {}

impl Renderable for PlaneEntity {
    fn is_emissive(&self) -> bool {
        !self.material.emission().is_zero()
    }

    fn get_random_emissive_surface(&self, rng: &mut StdRng) -> SurfaceDescription {
        let coordinate = self.plane.origin() +
            (self.plane.v() * self.extent * ((rng.gen::<f32>() * 2.0) - 1.0)) +
            (self.plane.u() * self.extent * ((rng.gen::<f32>() * 2.0) - 1.0));
        SurfaceDescription
        {
            coordinate,
            world_normal: self.plane.normal(),
            emission: *self.material.emission(),
            entity_id: self.entity_id,
        }
    }
}

impl Intersectable for PlaneEntity {
    fn intersect<'a>(&'a self, world_ray: &Ray) -> Option<Box<dyn Intersection + '_>> {
        if !ray_aabb_intersect(&world_ray, &self.bounds) {
            return None; // This is to confine the plane to a finite space.
        }
        let denominator = glm::dot(world_ray.direction, self.plane.normal());
            if glm::abs(denominator) < 0.0001 { // TODO: What epsilon value should I use?
            return None;
        }

        let x = self.plane.origin() - world_ray.origin;
        let t = glm::dot(x, self.plane.normal()) / denominator;

        Some(Box::new(PlaneIntersection {
            world_space_normal: self.plane.normal(),
            coordinate: world_ray.origin + (world_ray.direction * t),
            entity_id: self.entity_id,
            material: &self.material,
            distance: t
        }))
    }

    fn bounds(&self) -> &AABB {
        &self.bounds
    }

    fn entity_id(&self) -> u32 {
        self.entity_id
    }

    fn transform(&self) -> &Transform {
        &self.transform
    }
}

pub struct PlaneIntersection<'a> {
    entity_id: u32,
    coordinate: Vec3,
    world_space_normal: Vec3,
    material: &'a Material,
    distance: f32,
}

impl Intersection for PlaneIntersection<'_> {
    fn coordinate(&self) -> Vec3 {
        self.coordinate
    }

    fn world_space_normal(&self) -> Vec3 {
        self.world_space_normal
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