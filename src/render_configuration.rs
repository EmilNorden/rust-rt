use std::collections::HashMap;

mod parser;

pub use self::parser::ConfigurationParser;

pub struct RenderConfiguration {
    pub model_path_lookup: HashMap<String, String>,
    pub keyframes: Vec<KeyFrame>
}


pub struct KeyFrame {
    timestamp: f64,
    updates: Vec<KeyFrameUpdate>
}

pub struct KeyFrameUpdate {
    pub entity_name: String,
    pub model_name: Option<String>,
    pub position: Option<glm::Vec3>,
    pub rotation: Option<glm::Vec3>,
    pub scale: Option<glm::Vec3>,
}

impl KeyFrame {
    pub fn updates(&self) -> &Vec<KeyFrameUpdate> {
        &self.updates
    }

    pub fn updates_mut(&mut self) -> &mut Vec<KeyFrameUpdate> {
        &mut self.updates
    }

    pub fn timestamp(&self) -> f64 {
        self.timestamp
    }
}