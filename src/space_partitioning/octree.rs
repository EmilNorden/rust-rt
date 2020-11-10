use crate::core::{Ray, Intersection};
use crate::core::geom::{ray_aabb_intersect, AABB};
use crate::space_partitioning::{SpacePartitionSystem, BoundsCallback};

#[derive(Clone)]
struct OctantId { pub id: usize }

#[derive(Clone)]
struct EntityId { pub id: usize }

pub struct Octree<'a, T> {
    entities: Vec<&'a T>,
    octants: Vec<Octant>,
}

struct Octant {
    entities: Vec<EntityId>,
    children: Vec<OctantId>,
    bounds: AABB,
}

impl<'a, T> Octree<'a, T> {
    fn create(entities: &'a Vec<&'a T>, depth_limit: usize, bounds_cb: &BoundsCallback<T>) -> Octree<'a, &'a T> {
        let bounds = AABB::from_bounds(&entities.iter().map(|x| bounds_cb(x)));

        let root = Octant {
            entities: (0..entities.len()).map(|x| EntityId { id: x }).collect(),
            children: Vec::new(),
            bounds,
        };

        let mut tree = Octree {
            entities: entities.into_iter().map(|x| x).collect(),
            octants: vec![root],
        };

        // tree.split_octant(OctantId { id: 0 }, depth_limit, bounds_cb);

        tree
    }

    fn intersect(ray: &Ray) -> Option<Intersection> {
        unimplemented!()
    }

    fn split_octant(&mut self, current: OctantId, depth_limit: usize, bounds_cb: &BoundsCallback<T>) {
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
                        let entity: &T = &self.entities[x.id];
                        if bounds_cb(entity).intersects_bounds(&child_bounds) {
                            child_entities.push(x.clone());
                        }
                    }

                    // self.insert_new_octant(&current, child_entities, child_bounds, depth_limit);
                }
            }
        }

        self.octants[current.id].entities.clear();
    }
}