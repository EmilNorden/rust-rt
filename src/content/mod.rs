extern crate assimp;

mod mesh;


pub fn load(path: &str) -> Result<i32, &str> {
    let mut importer = assimp::import::Importer::new();
    setup_importer(&mut importer);

    let result = importer.read_file(path).unwrap();

    result.mat
    for x in result.material_iter() {
        x
    }

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