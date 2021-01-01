use crate::scene::{SceneEntity, Scene};
use crate::core::{Ray, Intersection};

pub struct NaiveScene {
    entities: Vec<SceneEntity>
}

#[allow(dead_code)]
impl NaiveScene {
    pub fn new() -> Self {
        NaiveScene {
            entities : Vec::new()
        }
    }
}

impl Scene for NaiveScene {
    fn trace(&self, ray: &Ray) -> Option<Intersection> {
        let mut result: Option<Intersection> = None;
        let mut best_distance = std::f32::MAX;
        for x in &self.entities {
            let transformed_ray = ray.transform(&x.inverse_transform);
            if let Some(intersection) = x.model.intersects(&transformed_ray) {
                if intersection.distance < best_distance {
                    best_distance = intersection.distance;
                    result = Some(intersection);
                }
            }
        }

        result
    }

    fn add(&mut self, entity: SceneEntity) {
        self.entities.push(entity);
    }
}

