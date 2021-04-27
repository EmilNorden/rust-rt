use crate::core::geom::{AABB, ray_aabb_intersect};
use crate::core::{Intersection, Ray};
use crate::scene::{Scene, SceneEntity};
use rand::rngs::StdRng;
use rand::Rng;

extern crate test;

#[derive(Clone)]
struct OctantId { pub id: usize }

#[derive(Clone)]
struct EntityId { pub id: usize }

pub struct Octree {
    entities: Vec<Box<dyn SceneEntity + Sync + Send>>,
    boundless_entities: Vec<Box<dyn SceneEntity + Sync + Send>>,
    octants: Vec<Octant>,
}

struct Octant {
    entities: Vec<EntityId>,
    children: Vec<OctantId>,
    bounds: AABB,
}

impl Scene for Octree {
    fn find_intersection(&self, ray: &Ray) -> Option<Box<dyn Intersection + '_>> {
        // self.trace_octant(ray, &OctantId { id: 0 })

        let octree_result = self.trace_octant(ray, &OctantId { id: 0 });

        let mut best_result: Option<Box<dyn Intersection>> = None;
        let mut best_distance = std::f32::MAX;
        for x in &self.boundless_entities {
            if let Some(intersection) = x.intersect(ray) {
                if best_result.is_none() {
                    best_result = Some(intersection);
                } else if best_distance > intersection.distance() {
                    best_distance = intersection.distance();
                    best_result = Some(intersection);
                }
            }
        }

        if octree_result.is_none() ||  best_distance < octree_result.as_ref().unwrap().distance() {
            best_result
        } else {
            octree_result
        }
        // octree_result

        /*let mut best_result: Option<Box<dyn Intersection>> = None;
        let mut best_distance = std::f32::MAX;
        for x in &self.boundless_entities {
            if let Some(intersection) = x.intersect(ray) {
                if best_result.is_none() {
                    best_distance = intersection.distance();
                    best_result = Some(intersection);
                } else if intersection.distance() < best_distance {
                    best_distance = intersection.distance();
                    best_result = Some(intersection);
                }
            }
        }

        best_result*/
    }

    /*fn get_random_emissive_surface(&self, rng: &mut StdRng) -> Box<dyn Intersection + '_> {
        let emissive_entities: Vec<&Box<dyn SceneEntity>> = self.entities
            .iter()
            .filter(|x| x.is_emissive())
            .collect();

        let random_entity = emissive_entities[rng.gen_range(0..emissive_entities.len())];

        random_entity.get_random_emissive_surface(rng)
    }*/

    fn get_emissive_entities(&self) -> Vec<&Box<dyn SceneEntity + Sync + Send>> {
        self.entities.iter().filter(|x| x.is_emissive()).collect()
    }
}

impl Octree {
    fn trace_octant(&self, ray: &Ray, octant_id: &OctantId) -> Option<Box<dyn Intersection + '_>> {
        let mut result: Option<Box<dyn Intersection>> = None;
        let octant = &self.octants[octant_id.id];
        if !ray_aabb_intersect(ray, &octant.bounds) {
            return None;
        }

        if octant.children.len() == 0 {
            let mut best_distance = std::f32::MAX;
            for x in &octant.entities {
                // let transformed_ray = ray.transform(&self.entities[x.id].inverse_transform);
                // let foo = &self.entities[x.id];
                // let bar = foo.intersect(ray);
                if let Some(intersection) = self.entities[x.id].intersect(ray) {
                    if intersection.distance() < best_distance {
                        best_distance = intersection.distance();
                        result = Some(intersection);
                    }
                }
            }
        }

        let mut best_distance = std::f32::MAX;
        for octant_id in &octant.children {
            if let Some(intersection) = self.trace_octant(ray, &octant_id) {
                if intersection.distance() < best_distance {
                    best_distance = intersection.distance();
                    result = Some(intersection);
                }
            }
        }

        result
    }

    pub fn trace(&self, ray: &Ray) -> Option<Box<dyn Intersection + '_>> {
        let octree_result = self.trace_octant(ray, &OctantId { id: 0 });

        let mut best_result: Option<Box<dyn Intersection>> = None;
        let mut best_distance = std::f32::MAX;
        for x in &self.boundless_entities {
            if let Some(intersection) = x.intersect(ray) {
                if best_result.is_none() {
                    best_result = Some(intersection);
                } else if best_distance > intersection.distance() {
                    best_distance = intersection.distance();
                    best_result = Some(intersection);
                }
            }
        }

        if best_distance < octree_result.as_ref().unwrap().distance() {
            best_result
        } else {
            octree_result
        }
    }

    pub fn create(mut entities: Vec<Box<dyn SceneEntity + Sync + Send>>, depth_limit: usize) -> Octree {
        let boundless_entities: Vec<Box<dyn SceneEntity + Sync + Send>> = entities
            .drain_filter(|x| x.bounds().is_none())
            .collect();

        let bounds = AABB::from_bounds(&entities.iter().map(|x| x.bounds().unwrap().clone()));

        let root = Octant {
            entities: (0..entities.len()).map(|x| EntityId { id: x }).collect(),
            children: Vec::new(),
            bounds,
        };

        let mut tree = Octree {
            entities,
            boundless_entities,
            octants: vec![root],
        };


        tree.split_octant(OctantId { id: 0 }, depth_limit);

        tree
    }

    fn split_octant(&mut self, current: OctantId, depth_limit: usize) {
        const ENTITY_THRESHOLD: usize = 4;

        // let octant = &self.octants[current.id];
        if self.octants[current.id].entities.len() < ENTITY_THRESHOLD || depth_limit == 0 {
            return;
        }

        let child_size = self.octants[current.id].bounds.size() / 2.0;
        for z in 0..2 {
            for y in 0..2 {
                for x in 0..2 {
                    let child_bounds = AABB::from_location_and_size(&(self.octants[current.id].bounds.min + (glm::Vector3::new(x as f32, y as f32, z as f32) * child_size)), &child_size);

                    let mut child_entities = Vec::new();
                    for x in &self.octants[current.id].entities {
                        let entity = &self.entities[x.id];
                        // if entity.model.bounds().transform(&glm::inverse(&entity.inverse_transform)).intersects_bounds(&child_bounds) {
                        if entity.bounds().unwrap().intersects_bounds(&child_bounds) {
                            child_entities.push(x.clone());
                        }
                    }

                    self.insert_new_octant(&current, child_entities, child_bounds, depth_limit);
                }
            }
        }

        self.octants[current.id].entities.clear();
    }

    fn insert_new_octant(&mut self, parent: &OctantId, entities: Vec<EntityId>, child_bounds: AABB, depth_limit: usize) {
        let child = Octant {
            entities,
            children: Vec::new(),
            bounds: child_bounds.clone(),
        };

        let child_index = self.octants.len();
        self.octants[parent.id].children.push(OctantId { id: child_index });
        self.octants.push(child);

        self.split_octant(OctantId { id: child_index }, depth_limit - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_single_mesh_intersection(b: &mut Bencher) {

        // TODO: How to avoid this ugly super stuff?
        // let _foo = super::super::super::content::load("/Users/emil/code/rust-rt/assets/models/apricot/Apricot_02_hi_poly.obj").unwrap();
        b.iter(|| {});
    }
}