use crate::core::geom::{AABB, ray_aabb_intersect, ray_triangle_intersect};
use crate::core::{Ray, Intersection};
use crate::scene::mesh_entity::MeshIntersection;

struct OctreeMeshOctant {
    indices: Vec<(u32, u32, u32)>,
    children: Vec<usize>,
    bounds: AABB,
}

pub struct OctreeMesh {
    octants: Vec<OctreeMeshOctant>,
    coordinates: Vec<glm::Vector3<f32>>,
    texcoords: Vec<glm::Vector2<f32>>,
    normals: Vec<glm::Vector3<f32>>,
    material_index: usize,
    name: String,
}

impl OctreeMesh {
    pub fn new(name: String, coordinates: Vec<glm::Vector3<f32>>, texcoords: Vec<glm::Vector2<f32>>, normals: Vec<glm::Vector3<f32>>, indices: Vec<(u32, u32, u32)>, material_index: usize) -> OctreeMesh {
        let mut octree = OctreeMesh {
            octants: vec![
                OctreeMeshOctant {
                    bounds: AABB::from_vector3(&coordinates),
                    children: Vec::new(),
                    indices,
                },
            ],
            coordinates,
            texcoords,
            normals,
            material_index,
            name,
        };

        octree.split_octant(0, 5);

        octree
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn material_index(&self) -> usize { self.material_index }

    pub fn calculate_texcoords(&self, indices: &(u32, u32, u32), u: f32, v: f32) -> glm::Vector2<f32> {
        let w = 1.0 - u - v;

        let texcoord1 = self.texcoords[indices.0 as usize];
        let texcoord2 = self.texcoords[indices.1 as usize];
        let texcoord3 = self.texcoords[indices.2 as usize];
        texcoord1 * w + texcoord2 * u + texcoord3 * v
    }

    pub fn calculate_object_space_normal(&self, indices: &(u32, u32, u32), u: f32, v: f32) -> glm::Vector4<f32> {
        let w = 1.0 - u - v;

        let normal1 = self.normals[indices.0 as usize];
        let normal2 = self.normals[indices.1 as usize];
        let normal3 = self.normals[indices.2 as usize];
        let interpolated_normal = normal1 * w + normal2 * u + normal3 * v;
        glm::Vector4::new(interpolated_normal.x, interpolated_normal.y, interpolated_normal.z, 0.0)
    }

    pub fn bounds(&self) -> &AABB {
        &self.octants[0].bounds
    }

    pub fn intersects(&self, ray: &Ray) -> Option<MeshIntersection> {
       self.intersects_octant(ray,0)
    }

    fn intersects_octant(&self, ray: &Ray, octant_index: usize) -> Option<MeshIntersection> {

        if !ray_aabb_intersect(ray, &self.octants[octant_index].bounds) {
            return None;
        }

        if self.octants[octant_index].children.is_empty() {
            let mut closest_distance = std::f32::MAX;
            let mut result_u = 0.0f32;
            let mut result_v = 0.0f32;
            let mut result_indices = (0, 0, 0);
            let mut found = false;

            for x in &self.octants[octant_index].indices {
                let mut distance = std::f32::MAX;
                let mut u = 0.0f32;
                let mut v = 0.0f32;
                if ray_triangle_intersect(ray, (self.coordinates[(*x).0 as usize], self.coordinates[(*x).1 as usize], self.coordinates[(*x).2 as usize]), &mut distance, &mut u, &mut v) {
                    if distance < closest_distance {
                        closest_distance = distance;
                        result_u = u;
                        result_v = v;
                        result_indices = *x;
                        found = true;
                    }
                }
            }

            if found {
                return Some(MeshIntersection {
                    mesh: &self,
                    material: None,
                    u: result_u,
                    v: result_v,
                    indices: result_indices,
                    distance: closest_distance,
                    material_index: self.material_index
                })
            }

            return None;
        }
        else {
            let mut best_distance = std::f32::MAX;
            let mut result: Option<MeshIntersection> = None;
            for child_index in &self.octants[octant_index].children {
                if let Some(intersection) = self.intersects_octant(ray, *child_index) {
                    if intersection.distance < best_distance {
                        best_distance = intersection.distance;
                        result = Some(intersection);
                    }
                }
            }

            return result;
        }

    }

    fn split_octant(&mut self, current_index: usize, depth_limit: usize) {
        const VERTEX_TRESHOLD: usize = 500;

        if self.octants[current_index].indices.len() < VERTEX_TRESHOLD || depth_limit == 0 {
            return;
        }

        let child_size = self.octants[current_index].bounds.size() / 2.0;
        for z in 0..2 {
            for y in 0..2 {
                for x in 0..2 {
                    let child_bounds = AABB::from_location_and_size(&(self.octants[current_index].bounds.min + (glm::Vector3::new(x as f32, y as f32, z as f32) * child_size)), &child_size);

                    let mut child_indices = Vec::new();

                    for x in &self.octants[current_index].indices {
                        let v1 = &self.coordinates[x.0 as usize];
                        let v2 = &self.coordinates[x.1 as usize];
                        let v3 = &self.coordinates[x.2 as usize];

                        if child_bounds.contains(v1) || child_bounds.contains(v2) || child_bounds.contains(v3) {
                            child_indices.push(*x);
                        }
                    }

                    self.insert_new_octant(current_index, child_indices, child_bounds, depth_limit);
                }
            }
        }
    }

    fn insert_new_octant(&mut self, parent_index: usize, indices: Vec<(u32, u32, u32)>, child_bounds: AABB, depth_limit: usize) {
        let child = OctreeMeshOctant {
            indices,
            children: Vec::new(),
            bounds: child_bounds.clone(),
        };

        let child_index = self.octants.len();
        self.octants[parent_index].children.push(child_index);
        self.octants.push(child);

        self.split_octant(child_index, depth_limit - 1);
    }
}