use crate::render_configuration::{KeyFrame, KeyFrameUpdate};
use num_traits::Zero;
use std::ops::Mul;

type MutValueCallback<T> = fn(&mut KeyFrameUpdate) -> &mut Option<T>;

pub struct KeyFrameInterpolator {

}

impl KeyFrameInterpolator {
    fn interpolate_vector<T: Mul<f32, Output = T>>(&self, start_timestamp: f64, end_timestamp: f64, current_timestamp: f64, from: T, to: T) -> T {
        let length = end_timestamp - start_timestamp;
        if length.is_zero() {
            return from;
        }

        let factor = ((current_timestamp - start_timestamp) / length) as f32;
        return to * factor;
    }

    fn set_current_value_for_property<T: Copy + Mul<f32, Output = T>>(&self, keyframes: &mut Vec<KeyFrame>, keyframe_index: usize, update_index: usize, value_fn: MutValueCallback<T>, default: T) {
        let entity_name = keyframes[keyframe_index].updates_mut()[update_index].entity_name.clone();
        if value_fn(&mut keyframes[keyframe_index].updates_mut()[update_index]).is_none() {
            let (previous_value, previous_timestamp) = self.find_previous_value_for_entity(&*entity_name, keyframes,keyframe_index, value_fn)
                .unwrap_or((default, 0.0));

            let (next_value, next_timestamp) = self.find_next_value_for_entity(&*entity_name, keyframes, keyframe_index, value_fn)
                .unwrap_or((previous_value, previous_timestamp));
            let interpolated_value = self.interpolate_vector(previous_timestamp, next_timestamp, keyframes[keyframe_index].timestamp(), previous_value, next_value);
            value_fn(&mut keyframes[keyframe_index].updates_mut()[update_index]).replace(interpolated_value);
        }
    }

    pub fn interpolate(&self, keyframes: &mut Vec<KeyFrame>) {
        for i in 0usize..keyframes.len() {
            for j in 0usize..keyframes[i].updates_mut().len() {
                let entity_name = keyframes[i].updates_mut()[j].entity_name.clone();

                self.set_current_value_for_property(keyframes, i, j, |x| &mut x.position, glm::Vec3::new(0.0, 0.0, 0.0));
                self.set_current_value_for_property(keyframes, i, j, |x| &mut x.rotation, glm::Vec3::new(0.0, 0.0, 0.0));
                self.set_current_value_for_property(keyframes, i, j, |x| &mut x.scale, glm::Vec3::new(1.0, 1.0, 1.0));
                /*if keyframes[i].updates_mut()[j].position.is_none() {
                    let (previous_value, previous_timestamp) = self.find_previous_value_for_entity(&*entity_name, keyframes,i, |x| x.position)
                        .unwrap_or(([0.0, 0.0, 0.0], 0.0));

                    let (next_value, next_timestamp) = self.find_next_value_for_entity(&*entity_name, keyframes, i, |x| x.position)
                        .unwrap_or((previous_value, previous_timestamp));
                    let interpolated_value = self.interpolate_vector(previous_timestamp, next_timestamp, keyframes[i].timestamp(), glm::Vec3(0.0, 0.0, 0.0), glm::Vec3(0.0, 0.0, 0.0));
                    //keyframes[i].updates_mut()[j].position = Some(interpolated_value);
                }

                if keyframes[i].updates_mut()[j].rotation.is_none() {
                    let (previous_value, previous_timestamp) = self.find_previous_value_for_entity(&*entity_name, keyframes,i, |x| x.rotation)
                        .unwrap_or(([0.0, 0.0, 0.0], 0.0));

                    let (next_value, next_timestamp) = self.find_next_value_for_entity(&*entity_name, keyframes, i, |x| x.rotation)
                        .unwrap_or((previous_value, previous_timestamp));
                    let interpolated_value = self.interpolate_vector(previous_timestamp, next_timestamp, keyframes[i].timestamp(), previous_value, next_value);
                    keyframes[i].updates_mut()[j].rotation = Some(interpolated_value);
                }

                if keyframes[i].updates_mut()[j].scale.is_none() {
                    let (previous_value, previous_timestamp) = self.find_previous_value_for_entity(&*entity_name, keyframes,i, |x| x.scale)
                        .unwrap_or(([0.0, 0.0, 0.0], 0.0));

                    let (next_value, next_timestamp) = self.find_next_value_for_entity(&*entity_name, keyframes, i, |x| x.scale)
                        .unwrap_or((previous_value, previous_timestamp));
                    let interpolated_value = self.interpolate_vector(previous_timestamp, next_timestamp, keyframes[i].timestamp(), previous_value, next_value);
                    keyframes[i].updates_mut()[j].scale = Some(interpolated_value);
                }*/
            }
        }
    }

    fn find_previous_value_for_entity<T: Copy>(&self, entity_name: &str, keyframes: &mut Vec<KeyFrame>, keyframe_index: usize, value_fn: MutValueCallback<T>) -> Option<(T, f64)> {
        for i in (0..keyframe_index).rev() {
            let keyframe = &mut keyframes[i];
            if let Some(update) = self.get_update_for_entity(entity_name, keyframe) {
                if value_fn(update).is_some() {
                    return Some((value_fn(update).unwrap(), keyframe.timestamp()));
                }
            }
        }

        None
    }

    fn find_next_value_for_entity<T: Copy>(&self, entity_name: &str, keyframes: &mut Vec<KeyFrame>, keyframe_index: usize, value_fn: MutValueCallback<T>) -> Option<(T, f64)> {
        for i in keyframe_index + 1..keyframes.len() {
            let keyframe = &mut keyframes[i];
            if let Some(update) = self.get_update_for_entity(entity_name, keyframe) {
                if value_fn(update).is_some() {
                    return Some((value_fn(update).unwrap(), keyframe.timestamp()));
                }
            }
        }

        None
    }

    fn get_update_for_entity<'a>(&self, entity_name: &str, keyframe: &'a mut KeyFrame) -> Option<&'a mut KeyFrameUpdate> {
        for update in keyframe.updates_mut() {
            if update.entity_name == entity_name {
                return Some(update);
            }
        }

        None
    }
}