use crate::core::{Ray, Intersection};
use crate::core::geom::AABB;

mod octree;

type BoundsCallback<T> = fn(item: &T) -> AABB;

pub trait SpacePartitionSystem<'a, T> {
    fn create(entities: &Vec<&'a T>, depth_limit: usize, bounds_cb: BoundsCallback<T>) -> Self;

    fn intersect(ray: &Ray) -> Option<Intersection>;
}