extern crate assimp;

use crate::content::mesh::{Model, Material, IndexedMesh};
use crate::core::geom::AABB;
use crate::content::mesh_split::{LinearMeshSplit, MeshSplit};
use std::convert::TryInto;

pub mod mesh;
pub mod mesh_loader;
pub mod mesh_split;
pub mod mesh_cache;

#[allow(dead_code)]
pub fn load_assimp(path: &str) -> Result<i32, &str> {
    /*let mut importer = assimp::import::Importer::new();
    setup_importer(&mut importer);

    let _result = importer.read_file(path).unwrap();

    let mut materials = vec![];
    let mut meshes = Vec::with_capacity(_result.num_meshes() as usize);
    for i in 0usize.._result.num_meshes().try_into().unwrap() {
        let mesh = _result.mesh(i).unwrap();

        let indices = tuples3(mesh.face_iter()).collect();
        let coordinates = tuples3(mesh.vertex_iter())
            .map(|tuple| glm::Vector3::new(tuple.0, tuple.1, tuple.2))
            .collect();

        let normals = tuples3(mesh.normal_iter())
            .map(|tuple| glm::Vector3::new(tuple.0, tuple.1, tuple.2))
            .collect();


        let bounds = AABB::from_vector3(&coordinates);
        let m = IndexedMesh {
            coordinates,
            normals,
            texcoords: mesh.texture_coords_iter(0).collect(),
            indices,
            bounds,
            material_id: match x.mesh.material_id {
                Some(id) => id + 1,
                None => 0
            } as u16
        };

        meshes.push(m);
    }*/
    Err("12")
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