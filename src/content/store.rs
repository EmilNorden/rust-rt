use std::collections::HashMap;
use crate::content::model::{Model, ModelInstance};
use crate::content::wavefront_model_loader::{WaveFrontObjectLoader};
use std::rc::Rc;
use crate::content::ModelLoader;

pub struct ModelStore {
    store: HashMap<String, Rc<Model>>,
    source: Box<dyn ModelLoader>,
}

impl ModelStore {
    pub fn new(source: Box<dyn ModelLoader>) -> ModelStore {
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