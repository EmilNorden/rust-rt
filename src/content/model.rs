use crate::content::octree_mesh::OctreeMesh;
use crate::content::material::Material;
use crate::core::geom::AABB;
use crate::core::{Intersection, Ray};

/*pub struct Mesh {
    pub coordinates: Vec<glm::Vector3<f32>>,
    pub texcoords: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<(u32, u32, u32)>,
    pub bounds: AABB,
    pub material_id: u16,
}*/

pub struct Model {
    pub meshes: Vec<OctreeMesh>,
    pub materials: Vec<Material>,
    pub bounds: AABB,
}

impl Model {
    pub fn new(meshes: Vec<OctreeMesh>, materials: Vec<Material>) -> Self {
        let first_child_bounds = match &meshes.first() {
            None => panic!("Cannot create a model with no meshes!"),
            Some(mesh) => mesh.bounds().clone()
        };
        let bounds =  meshes.iter().fold(first_child_bounds, | acc, x| AABB::combine(&acc, x.bounds()));
        Model {
            meshes,
            materials,
            bounds
        }
    }
    pub fn bounds(&self) -> &AABB {
        &self.bounds
    }

    pub fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let mut result: Option<Intersection> = None;
        let mut best_distance: f32 = std::f32::MAX;
        for mesh in &self.meshes {
            if let Some(mut mesh_result) = mesh.intersects(ray) {
                if mesh_result.distance < best_distance {
                    best_distance = mesh_result.distance;
                    mesh_result.material = self.materials.get(mesh_result.material_index);
                    result = Some(mesh_result);
                }
            }

        }
        result
    }
}

/*impl Mesh {
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
}*/