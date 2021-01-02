use std::collections::HashMap;
use crate::content::model::{Model, ModelInstance};
use crate::content::model_loader::{ModelLoader, LoadOptions};
use std::rc::Rc;

pub struct ModelStore {
    store: HashMap<String, Rc<Model>>,
    source: ModelLoader, // TODO: Create trait?
}

impl ModelStore {
    pub fn new(source: ModelLoader) -> ModelStore {
        ModelStore {
            store: HashMap::new(),
            source,
        }
    }

    pub fn load(&mut self, name: &str, path: &str) -> ModelInstance {
        let source = &mut self.source;

        ModelInstance::new(self.store.entry(name.to_string()).or_insert_with(||{
            Rc::new(source.load(path).unwrap())
        }).clone())
    }
}