extern crate assimp;

use crate::content::model::Model;

pub mod model;
pub mod wavefront_model_loader;
pub mod mesh;
pub mod octree_mesh;
pub mod material;
pub mod store;
pub mod assimp_model_loader;


pub trait ModelLoader {
    fn load(&self, path: &str) -> Result<Model, &str>;
}

#[allow(dead_code)]
pub fn load_assimp(_path: &str) -> Result<i32, &str> {
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