use crate::render_configuration::{Frame, PropertyValue, PropertyChanges};
use std::collections::{HashMap, HashSet};
use num_traits::Bounded;
use std::ops::RangeBounds;

pub struct FrameInterpolator<'a> {
    key_frames: &'a Vec<Frame>,
}

impl<'a> FrameInterpolator<'a> {
    pub fn new(key_frames: &'a Vec<Frame>) -> Self {
        FrameInterpolator {
            key_frames
        }
    }
}

struct PropertyInterpolation<T> {
    start_timestamp: f64,
    end_timestamp: f64,
    start_value: T,
    end_value: T,
}

struct PropProp<T> {
    setter: Box<dyn FnMut(T)>
}

impl FrameInterpolator<'_> {
    pub fn frame_at(&self, timestamp: f64) -> Frame {
        let frame_index = self.binary_search_for_frame_at_timestamp(timestamp, 0, self.key_frames.len() - 1);

        if frame_index == self.key_frames.len() - 1 {
            self.key_frames[frame_index].clone();
        }

        let mut from_values: HashMap<String, HashMap<String, (f64, PropertyValue)>> = HashMap::new();
        for i in (0..=frame_index).rev() {
            self.collect_property_values(&mut from_values, i)
        }

        let mut to_values: HashMap<String, HashMap<String, (f64, PropertyValue)>> = HashMap::new();
        for i in frame_index + 1..self.key_frames.len() {
            self.collect_property_values(&mut to_values, i);
        }

        let mut all_entity_ids: Vec<&String> = from_values.keys().chain(to_values.keys()).collect();
        all_entity_ids.sort();
        all_entity_ids.dedup();

        let mut all_changes = Vec::new();
        for id in all_entity_ids {
            let entity_from_values = &from_values[id];
            let entity_to_values = &to_values[id];

            let mut new_values = HashMap::new();
            for (property_name, property_value) in entity_from_values {
                let frame_value = match entity_to_values.get(property_name) {
                    Some(x) => (*x).1.lerp(&property_value.1, 1.0),
                    None => property_value.clone().1
                };

                new_values.insert(property_name.clone(), frame_value);
            }

            all_changes.push(PropertyChanges::new(id.clone(), new_values))
        }

        Frame::new(timestamp, all_changes)
    }

    fn collect_property_values(&self, from_values: &mut HashMap<String, HashMap<String, (f64, PropertyValue)>>, i: usize) {
        let frame = &self.key_frames[i];
        for entity in frame.updates() {
            let mut entity_properties = from_values.entry(entity.id().clone()).or_insert(HashMap::new());
            for (property_name, value) in entity.values() {
                if entity_properties.contains_key(property_name) {
                    continue;
                }

                entity_properties.insert(property_name.clone(), (frame.timestamp(), value.clone()));
            }
        }
    }


    fn binary_search_for_frame_at_timestamp(&self, timestamp: f64, start_index: usize, end_index: usize) -> usize {
        if start_index == end_index {
            return start_index;
        }

        let middle_index = (end_index - start_index) / 2;

        // We have landed on a frame that is AHEAD of the current timestamp
        if self.key_frames[middle_index].timestamp() > timestamp {
            return self.binary_search_for_frame_at_timestamp(timestamp, start_index, middle_index - 1);
        }

        // We have landed on a frame that is ON or BEFORE the current timestamp

        let foo = self.binary_search_for_frame_at_timestamp(timestamp, middle_index + 1, end_index);
        if self.key_frames[foo].timestamp() - timestamp <
            self.key_frames[middle_index].timestamp() - timestamp {
            return foo;
        }

        middle_index
    }
}