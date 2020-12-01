use crate::render_configuration::{RenderConfiguration, KeyFrame, KeyFrameUpdate};
use crate::content::model::Model;
use crate::content::store::ModelStore;
use image::Frame;
use std::collections::HashMap;
use crate::scene::SceneEntity;
use std::cmp::Ordering::{Less, Greater};
use crate::scene::octree_scene::Octree;
use std::thread::current;
use std::ops::Range;
use num_traits::Zero;

type Callback<T> = fn(&KeyFrameUpdate) -> Option<T>;

enum FindMode {
    Previous,
    Next,
}

pub struct FrameProcessor<'a> {
    key_frames: Vec<KeyFrame>,
    model_definitions: HashMap<String, String>,
    model_store: ModelStore,
    current_frame: usize,
    current_keyframe_index: usize,
    first_frame: bool,
    active_interpolations: Vec<Interpolation<'a>>,
    entity_models: HashMap<String, String>,
}

struct Interpolation<'a> {
    from: glm::Vec3,
    to: glm::Vec3,
    start_timestamp: f64,
    end_timestamp: f64,
    target_property: &'a mut Option<glm::Vec3>,
}

impl Interpolation<'_> {
    pub fn get_value(&self, current_timestamp: f64) -> glm::Vec3 {
        let length = self.end_timestamp - self.start_timestamp;
        if length.is_zero() {
            return self.from;
        }
        glm::mix_s(self.from, self.to, ((current_timestamp - self.start_timestamp) / length) as f32)
    }
}


impl FrameProcessor<'_> {
    pub fn new(key_frames: Vec<KeyFrame>, model_definitions: HashMap<String, String>, model_store: ModelStore) -> Self {
        if key_frames.is_empty() {
            panic!("key_frames cannot be empty");
        }

        let mut result = FrameProcessor {
            key_frames,
            model_store,
            current_frame: 0,
            current_keyframe_index: 0,
            first_frame: false,
            model_definitions,
            active_interpolations: Vec::new(),
            entity_models: HashMap::new(),
        };

        result
    }

    fn find_current_keyframe_index(&self, current_timestamp: f64) -> usize {
        let mut current_index = self.current_keyframe_index;
        while (current_index + 1) < self.key_frames.len() && self.key_frames[current_index + 1].timestamp() <= current_timestamp {
            current_index = current_index + 1;
        }

        current_index
    }

    pub fn tick(&mut self) -> bool {
        let frames_per_second: f64 = 60.0;
        let current_timestamp = self.current_frame as f64 / frames_per_second;

        let mut entities = HashMap::new();

        let entity_models = &mut self.entity_models;
        let model_store = &mut self.model_store;
        while self.current_keyframe_index < self.key_frames.len() - 1 && self.key_frames[self.current_keyframe_index + 1].timestamp() < current_timestamp {
            self.current_keyframe_index = self.current_keyframe_index + 1;

            for update in self.key_frames[self.current_keyframe_index].updates() {
                if update.model_name.is_some() {
                    entity_models.insert(update.entity_name.clone(), update.model_name.clone().unwrap());
                }

                let model_path = entity_models.get(&update.entity_name).expect("No model for entity!");
                let model = model_store.load(&update.entity_name, model_path);
                entities.entry(update.entity_name.clone()).or_insert(SceneEntity::new(model));
            }
        }

        let from_keyframe = &self.key_frames[self.current_keyframe_index];
        let to_keyframe = &self.key_frames[self.current_keyframe_index + 1];

        /*
Upprätthåll en lista av ActiveInterpolations (AI)
Gå igenom alla AIs och kolla om dom är relevanta. Det är dom om vi ännu inte uppnått end_timestamp.
Är dom inte relevanta, ta bort.
Om relevant:
    Utför interpolering och sätt rätt property.
        - Hur vet vi vilken property?
            - Alternativ:
                - Enum som beskriver property? Jobbigt att utöka.
                - En referens till Option<T>?

När vi påträffar en property-sättning:
 - Utför själva sättningen.
 - Kolla om det finns en senare sättning.
    Om det finns:
        - Skapa en AI

*/
        self.active_interpolations.drain_filter(|x| {
            let has_expired = x.end_timestamp < current_timestamp;
            if !has_expired {
                x.target_property.replace(x.get_value(current_timestamp));
            }
            has_expired
        });

        false
        /*
        Varje tick():
            - interpolera

         */
    }

    fn get_entity_model(&mut self, update: &KeyFrameUpdate) -> &Model {
        let model_name = update.model_name.as_ref()
            .expect(&*format!("First occurrence of entity '{}' must reference a model", update.entity_name));

        let model_path = self.model_definitions.get(model_name)
            .expect(&*format!("Entity '{}' references model '{}' which is not defined", update.entity_name, model_name));

        self.model_store.load(model_name, model_path)
    }
}