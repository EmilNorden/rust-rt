use crate::content::model::{Model};
use crate::content::octree_mesh::OctreeMesh;
use crate::content::material::{Texture, Material};
use std::path::Path;
use std::collections::HashMap;
use crate::content::ModelLoader;

pub struct WaveFrontObjectLoader {}


struct Tuples3<I> {
    original: I,
}

struct Tuples2<I> {
    original: I,
}

impl<I> Iterator for Tuples3<I> where I: Iterator {
    type Item = (I::Item, I::Item, I::Item);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t1) = self.original.next() {
            if let Some(t2) = self.original.next() {
                if let Some(t3) = self.original.next() {
                    return Some((t1, t2, t3));
                }
            }
        }

        return None;
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.original.size_hint() {
            (lower, Some(upper)) => (lower, Some(upper / 3)),
            h @ (_, _) => h
        }
    }
}

impl<I> Iterator for Tuples2<I> where I: Iterator {
    type Item = (I::Item, I::Item);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t1) = self.original.next() {
            if let Some(t2) = self.original.next() {
                return Some((t1, t2));
            }
        }

        return None;
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.original.size_hint() {
            (lower, Some(upper)) => (lower, Some(upper / 2)),
            h @ (_, _) => h
        }
    }
}

fn tuples3<I: Iterator>(iterator: I) -> Tuples3<I> {
    Tuples3 { original: iterator }
}

fn tuples2<I: Iterator>(iterator: I) -> Tuples2<I> {
    Tuples2 { original: iterator }
}

impl ModelLoader for WaveFrontObjectLoader
{
    fn load(&self, path: &str) -> Result<Model, &str> {
        let (models, materials) = tobj::load_obj(path).unwrap();

        println!("# of models: {}", models.len());
        println!("# of materials: {}", materials.len());

        let model_root = Path::new(path).parent().unwrap();

        let mut materials: Vec<Material> = materials.into_iter().map(|x| {
            let texture_relative_path = Path::new(x.diffuse_texture.as_str());
            let diffuse = Texture::from_file(model_root.join(texture_relative_path).to_str().unwrap());

            Material::new(Some(diffuse), glm::vec3(0.0, 0.0, 0.0), glm::vec3(0.0, 0.0, 0.0), 0.0)
        }).collect();

        let meshes = models.into_iter().map(|x|
            {
                let indices = tuples3(x.mesh.indices.into_iter()).collect();
                let coordinates = tuples3(x.mesh.positions.into_iter())
                    .map(|tuple| glm::Vector3::new(tuple.0, tuple.1, tuple.2))
                    .collect();

                let texcoords = tuples2(x.mesh.texcoords.into_iter())
                    .map(|tuple| glm::Vector2::new(tuple.0, tuple.1))
                    .collect();

                let normals = tuples3(x.mesh.normals.into_iter())
                    .map(|tuple| glm::Vector3::new(tuple.0, tuple.1, tuple.2))
                    .collect();

                // TODO: Replace unwrap() with pattern matching
                OctreeMesh::new(x.name, coordinates, texcoords, normals, indices, x.mesh.material_id.unwrap())
            }).collect();

        let result = Model::new(meshes, materials);

        println!("Loaded model {}", path);
        println!("Bounds: {}", result.bounds);

        //let split = LinearMeshSplit {};
        Ok(result)
        //Ok(split.split(result, 1000))
    }
}