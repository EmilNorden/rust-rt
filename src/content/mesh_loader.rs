use crate::content::mesh::{Model, IndexedMesh, Material};
use crate::core::geom::AABB;
use crate::content::mesh_split::{LinearMeshSplit, MeshSplit};

pub trait MeshLoader {
    fn load(&mut self, path: &str) -> Result<Model, &str>;
}

pub struct DefaultMeshLoader {}

struct Tuples<I> {
    original: I,
}

impl<I> Iterator for Tuples<I> where I: Iterator {
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

fn tuples3<I: Iterator>(iterator: I) -> Tuples<I> {
    Tuples { original: iterator }
}

impl MeshLoader for DefaultMeshLoader
{
    fn load(&mut self, path: &str) -> Result<Model, &str> {
        let (models, materials) = tobj::load_obj(path).unwrap();

        println!("# of models: {}", models.len());
        println!("# of materials: {}", materials.len());

        let meshes = models.into_iter().map(|x|
            {
                let indices = tuples3(x.mesh.indices.into_iter()).collect();
                let coordinates = tuples3(x.mesh.positions.into_iter())
                    .map(|tuple| glm::Vector3::new(tuple.0, tuple.1, tuple.2))
                    .collect();

                let bounds = AABB::from_vector3(&coordinates);

                IndexedMesh {
                    coordinates,
                    normals: x.mesh.normals,
                    texcoords: x.mesh.texcoords,
                    indices,
                    bounds,
                    material_id: match x.mesh.material_id {
                        Some(id) => id + 1,
                        None => 0
                    } as u16,
                }
            }).collect();

        let result = Model {
            materials: materials.into_iter().map(|x| { Material { id: 0 } }).collect(),
            meshes,
        };

        let split = LinearMeshSplit {};
        //Ok(result)
        Ok(split.split(result, 1000))
    }
}