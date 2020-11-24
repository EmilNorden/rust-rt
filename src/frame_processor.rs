use crate::scene_description::{SceneDescription, KeyFrame};
use crate::content::model::Model;
use crate::content::model_loader::ModelLoader;

struct FrameProcessor {
    scene_description: SceneDescription,
    model_loader: ModelLoader,

    current_keyframe: KeyFrame
}

impl FrameProcessor {
    // pub fn new(scene_description: SceneDescription, model_store)
}