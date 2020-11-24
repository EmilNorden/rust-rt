use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SceneDescription {
    models: Vec<ModelDefinition>,
    keyframes: Vec<KeyFrame>,
}

#[derive(Debug, Deserialize)]
pub struct ModelDefinition {
    name: String,
    path: String,
}

#[derive(Debug, Deserialize)]
pub struct KeyFrame {
    updates: Vec<KeyFrameUpdate>
}

#[derive(Debug, Deserialize)]
pub struct KeyFrameUpdate {
    pub entity_name: String,
    pub model_name: Option<String>,
    pub position: Option<[f32; 3]>,
    pub rotation: Option<[f32; 3]>,
    pub scale: Option<[f32; 3]>,
}

impl SceneDescription {
    pub fn keyframes(&self) -> &Vec<KeyFrame> {
        &self.keyframes
    }
}

impl KeyFrame {
    pub fn updates(&self) -> &Vec<KeyFrameUpdate> {
        &self.updates
    }
}