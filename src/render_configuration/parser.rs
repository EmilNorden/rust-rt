use serde::{Deserialize, Deserializer};
use std::io::BufReader;
use std::fs::File;
use crate::render_configuration::RenderConfiguration;

pub struct ConfigurationParser{}

impl ConfigurationParser {
    pub fn parse(&self, path: &str) -> RenderConfiguration {
        // "/Users/emil/code/rust-rt/src/test.json"
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let config: ParsedConfiguration = serde_json::from_reader(reader).unwrap();

        RenderConfiguration {
            keyframes: config.keyframes.iter().map(|x| {
                crate::render_configuration::KeyFrame {
                    timestamp: x.timestamp,
                    updates: x.updates.iter().map(|update| {
                        crate::render_configuration::KeyFrameUpdate {
                            scale: match update.scale {
                                Some(val) => Some(glm::Vec3::new(val[0], val[1], val[2])),
                                None => None
                            },
                            position: match update.position {
                                Some(val) => Some(glm::Vec3::new(val[0], val[1], val[2])),
                                None => None
                            },
                            rotation: match update.rotation {
                                Some(val) => Some(glm::Vec3::new(val[0], val[1], val[2])),
                                None => None
                            },
                            entity_name: update.entity_name.clone(),
                            model_name: update.model_name.clone()
                        }
                    }).collect()
                }
            }).collect(),
            model_path_lookup: config.models.iter().map(|x| (x.name.clone(), x.path.clone())).collect()
        }
    }
}

#[derive(Debug, Deserialize)]
struct ParsedConfiguration
{
    models: Vec<ModelFilePath>,
    keyframes: Vec<KeyFrame>,
}

#[derive(Debug, Deserialize)]
struct ModelFilePath {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Deserialize, Clone)]
struct KeyFrame {
    timestamp: f64,
    updates: Vec<KeyFrameUpdate>
}

#[derive(Debug, Deserialize, Clone)]
struct KeyFrameUpdate {
    pub entity_name: String,
    pub model_name: Option<String>,
    pub position: Option<[f32; 3]>,
    pub rotation: Option<[f32; 3]>,
    pub scale: Option<[f32; 3]>,
}

