use std::collections::HashMap;

pub mod parser;

// pub use self::parser::ConfigurationParser;
use crate::content::material::Material;

pub struct RenderConfiguration {
    pub shutter_speed: f64,
    pub duration: f64,
    pub frames_per_second: i32,
    pub model_path_lookup: HashMap<String, String>,
    pub entities: HashMap<String, EntityType>,
    pub keyframes: Vec<Frame>,
}

pub enum EntityType {
    Sphere,
    Model,
}

#[derive(Clone)]
pub struct Frame {
    timestamp: f64,
    updates: Vec<PropertyChanges>,
}

impl Frame {
    pub fn new(timestamp: f64, updates: Vec<PropertyChanges>) -> Self {
        Frame {
            timestamp,
            updates
        }
    }

    pub fn timestamp(&self) -> f64 { self.timestamp }
    pub fn updates(&self) -> &Vec<PropertyChanges> { &self.updates }
}

#[derive(Clone)]
pub struct PropertyChanges {
    id: String,
    values: HashMap<String, PropertyValue>,
}

impl PropertyChanges {
    pub fn new(id: String, values: HashMap<String, PropertyValue>) -> Self {
        PropertyChanges {
            id,
            values,
        }
    }

    pub fn id(&self) -> &String { &self.id }
    pub fn values(&self) -> &HashMap<String, PropertyValue> { &self.values }
}

#[derive(Clone)]
pub enum PropertyValue {
    Float(f32),
    Vec3(glm::Vec3),
    String(String),
}

impl PropertyValue {
    fn lerp_f32(a: f32, b: f32, factor: f32) -> f32 {
        a * (1.0 - factor) + b * factor
    }

    pub fn lerp(&self, other: &PropertyValue, factor: f32) -> PropertyValue {
        use PropertyValue::*;
        match (self, other) {
            (Float(a), Float(b)) => Float(Self::lerp_f32(*a, *b, factor)),
            (Vec3(a), Vec3(b)) => Vec3(glm::vec3(Self::lerp_f32(a.x, b.x, factor), Self::lerp_f32(a.y, b.y, factor), Self::lerp_f32(a.z, b.z, factor))),
            (String(a), String(b)) => String(a.clone()),
            _ => panic!("Cannot interpolate values of different types!")
        }
    }
}