extern crate assimp;

pub fn load(path: &str) -> i32 {
    let importer = assimp::import::Importer::new();
    importer.read_file(path)?;
}