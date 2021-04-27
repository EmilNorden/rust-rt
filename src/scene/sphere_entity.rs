use crate::scene::{Intersectable, Renderable, SceneEntity, SurfaceDescription};
use crate::core::{Intersection, Ray};
use crate::core::geom::AABB;
use glm::{Vec2, Vec3};
use crate::content::material::Material;
use num_traits::{Zero, One};
use rand::rngs::StdRng;
use rand::Rng;
use crate::scene::transform::Transform;

pub struct SphereIntersection<'a> {
    entity_id: u32,
    material: &'a Material,
    transform: &'a Transform,
    world_space_hit_point: glm::Vec3,
    object_space_normal: glm::Vec4,
    distance: f32,
}

impl Intersection for SphereIntersection<'_> {
    fn coordinate(&self) -> Vec3 {
        /*let tmp = *self.transform * glm::vec4(self.object_space_hit_point.x, self.object_space_hit_point.y, self.object_space_hit_point.z, 1.0);
        glm::vec3(tmp.x, tmp.y, tmp.z)*/
        self.world_space_hit_point
    }

    fn world_space_normal(&self) -> Vec3 {
        let tmp = *self.transform.world() * self.object_space_normal;
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
    radius: f32,
    squared_radius: f32,
    bounds: AABB,
    material: Material,
    transform: Transform,
}

impl SphereEntity {
    pub fn new(id: u32, radius: f32, material: Material, mut transform: Transform) -> Self {
        let bounds = AABB {
            min: glm::vec3(-radius, -radius, -radius),
            max: glm::vec3(radius, radius, radius),
        }.transform(transform.world());


        SphereEntity {
            entity_id: id,
            radius,
            squared_radius: radius * radius,
            bounds,
            material,
            transform,
        }
    }

    fn build_transform(position: &glm::Vec3, rotation: &glm::Vec3, scale: &glm::Vec3) -> glm::Mat4 {
        let mut transform = glm::ext::scale(&glm::Mat4::one(), *scale);
        transform = glm::ext::rotate(&transform, rotation.z, glm::vec3(0.0, 0.0, 1.0));
        transform = glm::ext::rotate(&transform, rotation.y, glm::vec3(0.0, 1.0, 0.0));
        transform = glm::ext::rotate(&transform, rotation.x, glm::vec3(1.0, 0.0, 0.0));
        glm::ext::translate(&transform, *position)
    }

    fn intersect_object_space_ray(&self, object_space_ray: &Ray) -> Option<f32> {
        // let oc = world_ray.origin - self.position;
        let L = glm::vec3(0.0, 0.0, 0.0) - object_space_ray.origin;
        let tca = glm::dot(L, object_space_ray.direction);

        // If sphere is behind ray
        /*if tca < 0.0 {
            return None;
        }*/

        let d2 = glm::dot(L, L) - tca * tca;

        if d2 > self.squared_radius {
            return None;
        }

        let thc = glm::sqrt(self.squared_radius - d2);
        let mut t0 = tca - thc;
        let mut t1 = tca + thc;

        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }

        if t0 < 0.0 {
            t0 = t1;
            if t0 < 0.0 {
                return None;
            }
        }

        return Some(t0);
    }
}

impl SceneEntity for SphereEntity {}

impl Renderable for SphereEntity {
    fn is_emissive(&self) -> bool {
        !self.material.emission().is_zero()
    }

    fn get_random_emissive_surface(&self, rng: &mut StdRng) -> SurfaceDescription {
        let point_on_unit_sphere = glm::vec3(
            rng.gen::<f32>() * 2.0 - 1.0,
            rng.gen::<f32>() * 2.0 - 1.0,
            rng.gen::<f32>() * 2.0 - 1.0);
        let surface_normal = *self.transform.world() * point_on_unit_sphere.extend(0.0);

        let coordinate = *self.transform.world() * (point_on_unit_sphere * self.radius).extend(1.0);

        SurfaceDescription {
            emission: *self.material.emission(),
            coordinate: coordinate.truncate(3),
            world_normal: surface_normal.truncate(3),
            entity_id: self.entity_id,
        }
    }

    /*fn get_random_emissive_surface(&self, rng: &mut StdRng) -> Box<dyn Intersection + '_> {
        let random_point = glm::normalize(glm::vec3(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            - glm::vec3(0.5, 0.5, 0.5)) * self.radius;

        Box::new(SphereIntersection {
            entity_id: self.entity_id,
            material: &self.material,
            transform: &self.transform,
            object_space_hit_point: random_point,
            sphere_position: self.position,
            distance: 0.0,
        })
    }*/
}

impl Intersectable for SphereEntity {
    fn intersect(&self, world_ray: &Ray) -> Option<Box<dyn Intersection + '_>> {
        let object_ray = world_ray.transform(&self.transform.inverse_world());

        if let Some(object_space_distance) = self.intersect_object_space_ray(&object_ray) {
            let object_space_hit_point = object_ray.origin + (object_ray.direction * object_space_distance);
            let tmp = *self.transform.world() * glm::vec4(object_space_hit_point.x, object_space_hit_point.y, object_space_hit_point.z, 1.0);
            let world_space_hit_point = glm::vec3(tmp.x, tmp.y, tmp.z);
            let world_space_distance = glm::length(world_space_hit_point - world_ray.origin);

            return Some(Box::new(SphereIntersection {
                entity_id: self.entity_id,
                material: &self.material,
                transform: &self.transform,
                world_space_hit_point: glm::vec3(world_space_hit_point.x, world_space_hit_point.y, world_space_hit_point.z),
                object_space_normal: glm::vec4(object_space_hit_point.x, object_space_hit_point.y, object_space_hit_point.z, 0.0),
                distance: world_space_distance,
            }));
        }

        None
    }

    fn bounds(&self) -> Option<&AABB> {
        Some(&self.bounds)
    }

    fn entity_id(&self) -> u32 {
        self.entity_id
    }

    fn transform(&self) -> &Transform { &self.transform }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::material_builder::MaterialBuilder;
    use crate::scene::transform_builder::TransformBuilder;
    use glm::is_approx_eq;
    use float_cmp::{F32Margin, ApproxEq};

    #[test]
    fn intersect_object_space_ray_simple_intersection() {
        let sphere = SphereEntity::new(0, 1.0, MaterialBuilder::new().build(), TransformBuilder::new().build());

        let ray = Ray {
            origin: glm::vec3(0.0, 0.0, 2.0),
            direction: glm::vec3(0.0, 0.0, -1.0),
        };

        let result = sphere.intersect_object_space_ray(&ray);

        assert_eq!(result.unwrap(), 1.0);
    }

    #[test]
    fn intersect_object_space_ray_should_not_intersect_behind_ray() {
        let sphere = SphereEntity::new(0, 1.0, MaterialBuilder::new().build(), TransformBuilder::new().build());

        let ray = Ray {
            origin: glm::vec3(0.0, 0.0, 2.0),
            direction: glm::vec3(0.0, 0.0, 1.0),
        };

        let result = sphere.intersect_object_space_ray(&ray);

        assert!(result.is_none())
    }

    #[test]
    fn intersect_object_space_ray_should_handle_rays_originating_inside_sphere() {
        let sphere = SphereEntity::new(0, 1.0, MaterialBuilder::new().build(), TransformBuilder::new().build());

        let ray = Ray {
            origin: glm::vec3(0.0, 0.0, 0.9),
            direction: glm::vec3(0.0, 0.0, 1.0),
        };

        let result = sphere.intersect_object_space_ray(&ray);
        assert!(result.unwrap().approx_eq(0.1, F32Margin { ulps: 2, epsilon: std::f32::EPSILON }))
    }
}