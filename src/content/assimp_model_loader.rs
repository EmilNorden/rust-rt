use crate::content::ModelLoader;
use crate::content::model::Model;
use crate::core::geom::AABB;
use crate::content::octree_mesh::OctreeMesh;
use super::assimp::Scene;
use crate::content::material::Material;

struct Tuples3<I> {
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

fn tuples3<I: Iterator>(iterator: I) -> Tuples3<I> {
    Tuples3 { original: iterator }
}

struct Tuples2<I> {
    original: I,
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

fn tuples2<I: Iterator>(iterator: I) -> Tuples2<I> {
    Tuples2 { original: iterator }
}

pub struct AssimpModelLoader {}

impl AssimpModelLoader {
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
}

impl AssimpModelLoader {
     fn parse_materials(scene: &Scene) -> Vec<Material> {
        for i in 0..scene.num_materials() as isize {
            let material = scene.material_iter().map(|x| unsafe {
                for j in 0..x.num_properties as isize {
                    let property = &**x.properties.offset(j);
                    print!("{} = ", property.key.as_ref());
                    match property.property_type {
                        AiPropertyTypeInfo::Float => {
                            assert_eq!(property.data_length, 4);
                            let b = [u8; 4];
                            b[0] = property.data[0];

                        }
                        AiPropertyTypeInfo::Double => {}
                        AiPropertyTypeInfo::String => {

                        }
                        AiPropertyTypeInfo::Integer => {}
                        AiPropertyTypeInfo::Buffer => {}
                    }
                    // println!("{} = {}", property.key.as_ref(), property.property_type)
                }
            });
        }

        let materials = Vec::new();
        materials
    }
}

impl ModelLoader for AssimpModelLoader {
    fn load(&self, path: &str) -> Result<Model, &str> {
        let mut importer = assimp::import::Importer::new();
        AssimpModelLoader::setup_importer(&mut importer);

        let _result = importer.read_file(path).unwrap();

        let materials = AssimpModelLoader::parse_materials(&_result);
        let mut meshes = Vec::with_capacity(_result.num_meshes() as usize);
        for i in 0.._result.num_meshes() as usize {
            let mesh = _result.mesh(i).unwrap();

            let mut indices = Vec::new();
            for face_index in 0..mesh.num_faces() {
                let face = mesh.get_face(face_index)
                    .expect(&*format!("Face {} returned 'None'. Incorrect model file?", face_index));

                assert_eq!(face.num_indices, 3);
                indices.push((face[0], face[1], face[2]));
            }
            /*let indices = mesh.face_iter()
                .map(|x| unsafe {
                    assert_eq!(x.num_indices, 3);
                    (*x.indices.offset(0), *x.indices.offset(1), *x.indices.offset(2))
                })
                .collect();*/
            let coordinates = mesh.vertex_iter()
                .map(|vec| glm::vec3(vec.x, vec.y, vec.z))
                .collect();

            let normals = mesh.normal_iter()
                .map(|vec| glm::vec3(vec.x, vec.y, vec.z))
                .collect();

            let tex_coords = mesh.texture_coords_iter(0)
                .map(|vec| glm::vec2(vec.x, vec.y))
                .collect();

            meshes.push(OctreeMesh::new(mesh.name.as_ref().to_string(), coordinates, tex_coords, normals, indices, mesh.material_index as usize));
        }

        Err("123")
    }
}