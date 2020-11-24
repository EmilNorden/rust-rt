use crate::scene::{SceneEntity, Scene};
use crate::core::{Ray, Intersection};

pub struct NaiveScene<'a> {
    entities: Vec<SceneEntity<'a>>
}

#[allow(dead_code)]
impl<'a> NaiveScene<'a> {
    pub fn new() -> NaiveScene<'a> {
        NaiveScene {
            entities : Vec::new()
        }
    }
}

impl<'a> Scene<'a> for NaiveScene<'a> {
    fn trace(&self, ray: &Ray) -> Option<Intersection> {
        let mut result: Option<Intersection> = None;
        let mut best_distance = std::f32::MAX;
        for x in &self.entities {
            let transformed_ray = ray.transform(&x.inverse_transform);
            if let Some(intersection) = x.mesh.intersects(&transformed_ray) {
                if intersection.distance < best_distance {
                    best_distance = intersection.distance;
                    result = Some(intersection);
                }
            }
        }

        result
    }

    fn add(&mut self, entity: SceneEntity<'a>) {
        self.entities.push(entity);
    }
}

