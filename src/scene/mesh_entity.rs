use crate::content::octree_mesh::OctreeMesh;
use crate::core::geom::AABB;
use crate::content::material::Material;
use crate::core::{Intersection, Ray};
use glm::{Vec2, Vec3};
use crate::scene::Intersectable;

pub struct MeshIntersection<'a> {
    pub(crate) mesh: &'a OctreeMesh,
    pub material: Option<&'a Material>,
    pub u: f32,
    pub v: f32,
    pub indices: (u32, u32, u32),
    pub distance: f32,
    pub material_index: usize,
}

/*impl Intersection for MeshIntersection<'_> {
    fn object_space_normal(&self) -> Vec3 {
        let normal = self.mesh.calculate_object_space_normal(&self.indices, self.u, self.v);
        glm::vec3(normal.x, normal.y, normal.z)
    }

    fn texture_coordinates(&self) -> Vec2 {
        self.mesh.calculate_texcoords(&self.indices, self.u, self.v)
    }

    fn material(&self) -> &Material {
        self.material.unwrap()
    }

    fn distance(&self) -> f32 {
        self.distance
    }
}*/
/*
pub struct MeshEntity {
    mesh: OctreeMesh,
    pub inverse_transform: glm::Mat4,
    bounds: AABB,
}

impl Intersectable for MeshEntity {
    fn intersect<'a>(&'a self, world_ray: &Ray) -> Option<Box<dyn Intersection + 'a>> {
        let object_ray = world_ray.transform(&self.inverse_transform);
        match self.mesh.intersects(&object_ray) {
            Some(x) => Some(Box::new(x)),
            None => None
        }
    }

    fn bounds(&self) -> &AABB {
        &self.bounds
    }
}
/
 */