use crate::content::mesh::{Model, IndexedMesh};
use crate::core::geom::AABB;
use std::collections::{HashSet, HashMap};
use glm::Vector3;

pub trait MeshSplit
{
    fn split(&self, model: Model, vertex_limit: usize) -> Model;
}

pub struct LinearMeshSplit;

impl LinearMeshSplit {
    fn split_mesh(&self, mut mesh: IndexedMesh, vertex_limit: usize) -> Vec<IndexedMesh> {

        let mut new_meshes = Vec::new();
        let mut new_coordinates = Vec::new();
        let mut new_normals = Vec::new();
        let mut new_texcoords = Vec::new();
        let mut new_indices = Vec::new();
        let mut new_indices_set = HashSet::<u32>::new();
        let mut old_to_new_index_map = HashMap::<u32, u32>::new();

        let mut found_adjacent_triangles = true;
        while !mesh.indices.is_empty() {
            println!("mesh contains: {} indices", mesh.indices.len());

            if !found_adjacent_triangles {
                LinearMeshSplit::InsertNewMesh(&mut mesh, &mut new_meshes, &mut new_coordinates, &mut new_normals, &mut new_texcoords, &mut new_indices, &mut new_indices_set);
            }

            found_adjacent_triangles = false;
            for i in (0..mesh.indices.len()).rev() {
                if new_coordinates.len() + 3 > vertex_limit {
                    LinearMeshSplit::InsertNewMesh(&mut mesh, &mut new_meshes, &mut new_coordinates, &mut new_normals, &mut new_texcoords, &mut new_indices, &mut new_indices_set);
                }

                let tri_indices = mesh.indices[i];


                let index0_exists = new_indices_set.contains(&tri_indices.0);
                let index1_exists = new_indices_set.contains(&tri_indices.1);
                let index2_exists = new_indices_set.contains(&tri_indices.2);
                if new_indices_set.is_empty() || index0_exists || index1_exists || index2_exists {

                    found_adjacent_triangles = true;

                    mesh.indices.remove(i);

                    if !index0_exists {
                        new_indices_set.insert(tri_indices.0);
                        old_to_new_index_map.insert(tri_indices.0, new_coordinates.len() as u32);
                        new_coordinates.push(mesh.coordinates[tri_indices.0 as usize]);
                        new_normals.push(mesh.normals[tri_indices.0 as usize]);
                        new_texcoords.push(mesh.texcoords[tri_indices.0 as usize]);
                    }

                    if !index1_exists {
                        new_indices_set.insert(tri_indices.1);
                        old_to_new_index_map.insert(tri_indices.1, new_coordinates.len() as u32);
                        new_coordinates.push(mesh.coordinates[tri_indices.1 as usize]);
                        new_normals.push(mesh.normals[tri_indices.1 as usize]);
                        new_texcoords.push(mesh.texcoords[tri_indices.1 as usize]);
                    }

                    if !index2_exists {
                        new_indices_set.insert(tri_indices.2);
                        old_to_new_index_map.insert(tri_indices.2, new_coordinates.len() as u32);
                        new_coordinates.push(mesh.coordinates[tri_indices.2 as usize]);
                        new_normals.push(mesh.normals[tri_indices.2 as usize]);
                        new_texcoords.push(mesh.texcoords[tri_indices.2 as usize]);
                    }

                    new_indices.push((old_to_new_index_map[&tri_indices.0], old_to_new_index_map[&tri_indices.1], old_to_new_index_map[&tri_indices.2]));
                }
            }

            /*if !new_indices.is_empty() {
                LinearMeshSplit::InsertNewMesh(&mut mesh, &mut new_meshes, &mut new_coordinates, &mut new_normals, &mut new_texcoords, &mut new_indices, &mut new_indices_set);
            }*/
        }

        new_meshes
    }

    fn InsertNewMesh(mut mesh: &mut IndexedMesh, new_meshes: &mut Vec<IndexedMesh>, mut new_coordinates: &mut Vec<Vector3<f32>>, mut new_normals: &mut Vec<f32>, mut new_texcoords: &mut Vec<f32>, mut new_indices: &mut Vec<(u32, u32, u32)>, new_indices_set: &mut HashSet<u32>) {

        println!("# creating mesh with vertex count {}", new_coordinates.len());
        new_meshes.push(IndexedMesh {
            coordinates: (*new_coordinates).clone(),
            material_id: mesh.material_id,
            indices: (*new_indices).clone(),
            normals: (*new_normals).clone(),
            texcoords: (*new_texcoords).clone(),
            bounds: AABB::from_vector3(new_coordinates),
        });
        new_coordinates.clear();
        new_normals.clear();
        new_texcoords.clear();
        new_indices.clear();
        new_indices_set.clear();
    }
}

impl MeshSplit for LinearMeshSplit {
    fn split(&self, model: Model, vertex_limit: usize) -> Model {
        let mut new_meshes = Vec::new();

        println!("# of meshes before split: {}", model.meshes.len());
        for mesh in model.meshes {
            new_meshes.append(&mut self.split_mesh(mesh, vertex_limit));
        }

        println!("# of meshes after split: {}", new_meshes.len());

        Model {
            materials: model.materials,
            meshes: new_meshes
        }
    }
}