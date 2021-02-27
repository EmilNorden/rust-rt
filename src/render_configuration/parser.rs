use crate::render_configuration::{RenderConfiguration, Frame, PropertyChanges, PropertyValue};
use std::fs::File;
use std::io::Read;
use serde_json::{Value, Number, Map};
use std::collections::HashMap;

fn get_f32(node: &Value) -> Option<f32> {
    match node.as_f64() {
        Some(x) => Some(x as f32),
        None => None,
    }
}

pub fn parse<R>(reader: R) -> Result<RenderConfiguration, &'static str>
    where
        R: Read
{
    let root: Value = serde_json::from_reader(reader).unwrap();

    if !root["keyframes"].is_array() {
        return Err("'keyframes' must be an array");
    }

    if !root["scene"].is_object() {
        return Err("Missing required object 'scene' at root level");
    }

    let scene = root["scene"].as_object().unwrap();

    if !scene["duration"].is_f64() {
        return Err("'duration' must be a number");
    }

    if !scene["frames_per_second"].is_u64() {
        return Err("'frames_per_second' must be a whole number");
    }

    if !scene["shutter_speed"].is_object() {
        return Err("Missing required object 'shutter_speed' at scene level");
    }

    let shutter_speed = scene["shutter_speed"].as_object().unwrap();

    if !shutter_speed["numerator"].is_f64() {
        return Err("Missing required field 'numerator' on shutter_speed level");
    }

    if !shutter_speed["denominator"].is_f64() {
        return Err("Missing required field 'numerator' on shutter_speed level");
    }

    Ok(RenderConfiguration {
        shutter_speed: shutter_speed["numerator"].as_f64().unwrap() / shutter_speed["denominator"].as_f64().unwrap(),
        duration: scene["duration"].as_f64().unwrap(),
        frames_per_second: scene["frames_per_second"].as_u64().unwrap() as i32,
        model_path_lookup: get_model_path_lookup(&root)?,
        keyframes: get_keyframes(&root)?,
        entities: Default::default(),
    })
}

fn get_model_path_lookup(root_node: &Value) -> Result<HashMap<String, String>, &'static str> {
    let models_node = &root_node["models"];

    if !models_node.is_array() {
        return Err("Expected 'models' to be an array");
    }

    let mut lookup = HashMap::new();
    for model in models_node.as_array().unwrap() {
        if !model.is_object() {
            return Err("Expected 'models' array to contain objects only");
        }

        if !model["name"].is_string() {
            return Err("Model must contain a name");
        }

        if !model["path"].is_string() {
            return Err("Model must contain a path");
        }

        lookup.insert(model["name"].as_str().unwrap().to_string(), model["path"].as_str().unwrap().to_string());
    }

    Ok(lookup)
}

fn get_keyframes(root_node: &Value) -> Result<Vec<Frame>, &'static str> {
    let mut frames = Vec::new();
    for kf in root_node["keyframes"].as_array().unwrap() {
        let timestamp = kf["timestamp"].as_f64();

        if timestamp.is_none() {
            return Err("Each keyframe must specify a timestamp");
        }

        frames.push(
            Frame::new(timestamp.unwrap(), get_keyframe_updates(&kf["updates"])?));
    }

    Ok(frames)
}

fn get_keyframe_updates(updates_node: &Value) -> Result<Vec<PropertyChanges>, &'static str> {
    let mut result = Vec::new();

    if !updates_node.is_array() {
        return Err("Expected 'updates' to be an array");
    }

    let updates = updates_node.as_array().unwrap();

    for update in updates {
        if !update.is_object() {
            return Err("Expected 'updates' array to contain objects only");
        }

        let mut id: Option<String> = None;
        let mut properties = HashMap::new();
        for entry in update.as_object().unwrap() {
            if entry.0 == "id" {
                if !entry.1.is_string() {
                    return Err("'id' must be a string");
                }

                id = Some(entry.1.as_str().unwrap().to_string());
                continue;
            }

            properties.insert(entry.0.clone(), get_property_value(entry.1));
        }

        if id.is_none() {
            return Err("Each update must specify an id");
        }

        result.push(PropertyChanges::new(id.unwrap(), properties));
    }


    Ok(result)
}

fn get_property_value(value: &Value) -> PropertyValue {
    match value {
        Value::String(x) => PropertyValue::String(x.clone()),
        Value::Number(x) => PropertyValue::Float(x.as_f64().unwrap() as f32),
        Value::Array(x)if x.len() == 3 && x.iter().all(|y| y.is_f64())
        => PropertyValue::Vec3(glm::vec3(x[0].as_f64().unwrap() as f32, x[1].as_f64().unwrap() as f32, x[2].as_f64().unwrap() as f32)),
        _ => panic!("WHAT")
    }
}