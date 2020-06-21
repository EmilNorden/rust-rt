extern crate assimp;

use crate::content::mesh::{Model, Material, IndexedMesh};
use crate::core::geom::AABB;

pub mod mesh;

pub struct Tuples<I> {
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

pub fn tuples<I: Iterator>(iterator: I) -> Tuples<I> {
    Tuples { original: iterator }
}

pub fn load(path: &str) -> Result<Model, &str> {
    let (models, materials) = tobj::load_obj(path).unwrap();

    println!("# of models: {}", models.len());
    println!("# of materials: {}", materials.len());

    let meshes = models.into_iter().map(|x|
        {
            /*let mut indices = Vec::<(u32, u32, u32)>::with_capacity(x.mesh.indices.len() / 3);

            for i in (0..x.mesh.indices.len()).step_by(3) {
                indices.push((x.mesh.indices[i], x.mesh.indices[i + 1], x.mesh.indices[i + 2]));
            }*/

            let indices = tuples(x.mesh.indices.into_iter()).collect();
            let coordinates = tuples(x.mesh.positions.into_iter())
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

    Ok(result)
}

#[allow(dead_code)]
pub fn load_assimp(path: &str) -> Result<i32, &str> {
    let mut importer = assimp::import::Importer::new();
    setup_importer(&mut importer);

    let _result = importer.read_file(path).unwrap();

    Ok(32)
}

fn setup_importer(importer: &mut assimp::import::Importer) {
    importer.calc_tangent_space(|v| {
        v.enable = true;
    });

    importer.triangulate(true);

    importer.improve_cache_locality(|v| {
        v.enable = true;
    });

    importer.remove_redudant_materials(|v| {
        v.enable = true;
    });

    importer.optimize_meshes(true);
    importer.optimize_graph(|v| {
        v.enable = true;
    });

    importer.fix_infacing_normals(true);
    importer.find_invalid_data(|v| {
        v.enable = true;
    });

    // Should I remove this? Do I really want to use index buffer?
    importer.join_identical_vertices(true);

    importer.find_instances(true);

    importer.gen_uv_coords(true);
    importer.sort_by_primitive_type(|v| {
        v.enable = true;
    });

    importer.generate_normals(|v| {
        v.enable = true;
        v.smooth = true;
    });

    // Use or not?
    importer.split_large_meshes(|v| {
        v.enable = true;
    });
}