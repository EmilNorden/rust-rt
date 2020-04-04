use crate::core::{Ray, Intersection };
use crate::core::geom::{ray_triangle_intersect, AABB, ray_aabb_intersect};

pub struct IndexedMesh {
    pub coordinates: Vec<glm::Vector3<f32>>,
    pub texcoords: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<(u32, u32, u32)>,
    pub bounds: AABB,
    pub material_id: u16,
}

pub struct Material {
    pub id: u16
}

pub struct Model {
    pub meshes: Vec<IndexedMesh>,
    pub materials: Vec<Material>,
}

impl IndexedMesh {
    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {

        if !ray_aabb_intersect(&ray, &self.bounds) {
            return None;
        }

        let mut closest_distance = std::f32::MAX;
        let mut result_u = 0.0f32;
        let mut result_v = 0.0f32;
        let mut found = false;

        for x in &self.indices {
            let mut distance = std::f32::MAX;
            let mut u = 0.0f32;
            let mut v = 0.0f32;
            if ray_triangle_intersect(ray, (self.coordinates[(*x).0 as usize], self.coordinates[(*x).1 as usize], self.coordinates[(*x).2 as usize]), &mut distance, &mut u, &mut v) {
                if distance < closest_distance {
                    closest_distance = distance;
                    result_u = u;
                    result_v = v;
                    found = true;
                }
            }
        }

        if found {
            return Some(Intersection {
                u: result_u,
                v: result_v,
                distance: closest_distance
            });
        }

        None
    }
}