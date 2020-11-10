use std::path::PathBuf;
use crate::content::mesh::Model;
use std::collections::HashMap;


pub trait ContentCache<T>
{
    fn get(&self, path: &PathBuf) -> Option<&T>;
    fn remove(&mut self, path: &PathBuf);
    fn insert(&mut self, path: &PathBuf, content: T);
}

pub struct DefaultContentCache<T> {
    map: HashMap<PathBuf, T>
}

impl DefaultContentCache<T> {
    pub fn new<T>() -> DefaultContentCache<T> {
        DefaultContentCache {
            map: HashMap::new()
        }
    }
}

impl ContentCache<T> for DefaultContentCache<T> {
    fn get(&self, path: &PathBuf) -> Option<&T> {
        self.map.get(path)
    }

    fn remove(&mut self, path: &PathBuf) {
        self.remove(path)
    }

    fn insert(&mut self, path: &PathBuf, content: T) {
        self.map.insert(path.clone(), content);
    }
}

#[test]
fn get_nonexisting_key_should_return_none() {
    let mut c = DefaultMeshCache::new();

    let result = c.get(&PathBuf::from("thisisatest"));

    assert!(result.is_none())
}

fn get_existing_key_should_return_value() {
    let model1 = Model{ meshes: vec![], materials: vec![] };
    let model2 = Model{ meshes: vec![], materials: vec![] };




}
