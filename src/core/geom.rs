use super::*;
use num_traits::real::Real;

struct AABB {
    pub min: glm::Vector3<f32>,
    pub max: glm::Vector3<f32>,
}

impl AABB {
    pub fn from_vector4<I>(vertices: &I) -> AABB where I : IntoIterator<Item = glm::Vector4<f32>>
    {
        let mut smallest = glm::Vector3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX);
        let mut largest = glm::Vector3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN);

        for vertex in vertices.into_iter() {
            if vertex.x < smallest.x {
                smallest.x = vertex.x;
            }

            if vertex.y < smallest.y {
                smallest.y = vertex.y;
            }

            if vertex.z < smallest.z {
                smallest.z = vertex.z;
            }

            if vertex.x > largest.x {
                largest.x = vertex.x;
            }

            if vertex.y > largest.y {
                largest.y = vertex.y;
            }

            if vertex.z > largest.z {
                largest.z = vertex.z;
            }
        }

        AABB {
            min: smallest,
            max: largest,
        }
    }

    pub fn transform(&self, mat: &glm::Mat4) -> AABB {
        let corners: [glm::Vector4<f32>; 8] = [
            *mat * glm::Vector4 { x: self.min.x, y: self.min.y, z: self.min.z, w: 1.0 },
            *mat * glm::Vector4 { x: self.min.x, y: self.min.y, z: self.max.z, w: 1.0 },
            *mat * glm::Vector4 { x: self.min.x, y: self.max.y, z: self.min.z, w: 1.0 },
            *mat * glm::Vector4 { x: self.max.x, y: self.min.y, z: self.min.z, w: 1.0 },

            *mat * glm::Vector4 { x: self.max.x, y: self.max.y, z: self.max.z, w: 1.0 },
            *mat * glm::Vector4 { x: self.max.x, y: self.max.y, z: self.min.z, w: 1.0 },
            *mat * glm::Vector4 { x: self.max.x, y: self.min.y, z: self.max.z, w: 1.0 },
            *mat * glm::Vector4 { x: self.min.x, y: self.max.y, z: self.max.z, w: 1.0 },
        ];

        AABB::from_vector4(&corners.iter())

        /*let mut smallest = glm::Vector3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX);
        let mut largest = glm::Vector3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN);
        for corner in &corners {
            if corner.x < smallest.x {
                smallest.x = corner.x;
            }

            if corner.y < smallest.y {
                smallest.y = corner.y;
            }

            if corner.z < smallest.z {
                smallest.z = corner.z;
            }

            if corner.x > largest.x {
                largest.x = corner.x;
            }

            if corner.y > largest.y {
                largest.y = corner.y;
            }

            if corner.z > largest.z {
                largest.z = corner.z;
            }
        }

        AABB {
            min: smallest,
            max: largest,
        }*/
    }
}

pub fn triangle_intersect(ray: &Ray, triangle: (glm::Vector3<f32>, glm::Vector3<f32>, glm::Vector3<f32>), distance: &mut f32, result_u: &mut f32, result_v: &mut f32) -> bool {
    const EPSILON: f32 = 9.99999997475243E-07;

    // Find vectors for two edges sharing V1
    let e1 = triangle.1 - triangle.0;
    let e2 = triangle.2 - triangle.0;

    // Begin calculating determinant - also used to calculate u parameter
    let p = glm::cross(ray.direction, e2);
    // if determinant is near zero, ray lies in plane of triangle

    let determinant = glm::dot(e1, p);

    if determinant > -EPSILON && determinant < EPSILON {
        return false;
    }

    /*if determinant < EPSILON {
        return false;
    }*/

    let inv_determinant = 1.0 / determinant;

    // calculate distance from V1 to ray origin
    let t = ray.origin - triangle.0;

    // calculate u parameter and test bound
    let u = glm::dot(t, p) * inv_determinant;

    // The intersection lies outside of the triangle
    if u < 0.0 || u > 1.0 {
        return false;
    }

    // Prepare to test v parameter
    let q = glm::cross(t, e1);

    // calculate v parameter and test bound
    let v = glm::dot(ray.direction, q) * inv_determinant;

    // The intersection ies outside of the triangle
    if v < 0.0 || (u + v) > 1.0 {
        return false;
    }

    let t = glm::dot(e2, q) * inv_determinant;
    if t > EPSILON {
        *distance = t;
        *result_u = u;
        *result_v = v;
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_intersect_should_handle_simple_intersection() {
        let mut ray = Ray {
            origin: glm::Vector3::new(0.0, 0.0, 0.0),
            direction: glm::Vector3::new(0.0, 0.0, -1.0),
        };

        let triangle = (
            glm::Vector3::new(0.0, 0.5, -1.0),
            glm::Vector3::new(0.5, -0.5, -1.0),
            glm::Vector3::new(-0.5, -0.5, -1.0)
        );

        let mut distance = 0.0f32;
        let mut u = 0.0f32;
        let mut v = 0.0f32;

        let result = triangle_intersect(&ray, triangle, &mut distance, &mut u, &mut v);
        assert_eq!(result, true);
    }

    #[test]
    fn triangle_intersect_should_handle_backface_intersection() {
        let mut ray = Ray {
            origin: glm::Vector3::new(0.0, 0.0, 0.0),
            direction: glm::Vector3::new(0.0, 0.0, -1.0),
        };

        let triangle = (
            glm::Vector3::new(0.0, 0.5, -1.0),
            glm::Vector3::new(-0.5, -0.5, -1.0),
            glm::Vector3::new(0.5, -0.5, -1.0)
        );

        let mut distance = 0.0f32;
        let mut u = 0.0f32;
        let mut v = 0.0f32;

        let result = triangle_intersect(&ray, triangle, &mut distance, &mut u, &mut v);
        assert_eq!(result, true);
    }

    #[test]
    fn triangle_intersect_should_handle_simple_miss() {
        let mut ray = Ray {
            origin: glm::Vector3::new(0.5, 0.5, 0.0),
            direction: glm::Vector3::new(0.0, 0.0, -1.0),
        };

        let triangle = (
            glm::Vector3::new(0.0, 0.5, -1.0),
            glm::Vector3::new(0.5, -0.5, -1.0),
            glm::Vector3::new(-0.5, -0.5, -1.0)
        );

        let mut distance = 0.0f32;
        let mut u = 0.0f32;
        let mut v = 0.0f32;

        let result = triangle_intersect(&ray, triangle, &mut distance, &mut u, &mut v);
        assert_eq!(result, false);
    }

    #[test]
    fn triangle_intersect_should_handle_scewed_triangle() {
        let mut ray = Ray {
            origin: glm::Vector3::new(0.0, 0.0, 0.0),
            direction: glm::Vector3::new(0.0, 1.0, 0.0),
        };

        let triangle = (
            glm::Vector3::new(0.0, 10.0, 0.5),
            glm::Vector3::new(0.5, 13.0, -0.5),
            glm::Vector3::new(-0.5, 7.0, -0.5)
        );

        let mut distance = 0.0f32;
        let mut u = 0.0f32;
        let mut v = 0.0f32;

        let result = triangle_intersect(&ray, triangle, &mut distance, &mut u, &mut v);
        assert_eq!(result, true);
    }

    #[test]
    fn triangle_intersect_should_handle_parallel_triangle_miss() {
        let mut ray = Ray {
            origin: glm::Vector3::new(0.0, 0.0, 0.0),
            direction: glm::Vector3::new(1.0, 0.0, 0.0),
        };

        let triangle = (
            glm::Vector3::new(1.0, 0.0, 0.0),
            glm::Vector3::new(2.0, 0.0, 0.5),
            glm::Vector3::new(2.0, 0.0, -0.5)
        );

        let mut distance = 0.0f32;
        let mut u = 0.0f32;
        let mut v = 0.0f32;

        let result = triangle_intersect(&ray, triangle, &mut distance, &mut u, &mut v);
        assert_eq!(result, false);
    }

    #[test]
    fn triangle_intersect_should_return_correct_distance() {
        let mut ray = Ray {
            origin: glm::Vector3::new(0.0, 0.0, 0.0),
            direction: glm::Vector3::new(0.0, 0.0, -1.0),
        };

        let triangle = (
            glm::Vector3::new(0.0, 0.5, -5.0),
            glm::Vector3::new(0.5, -0.5, -5.0),
            glm::Vector3::new(-0.5, -0.5, -5.0)
        );

        let mut distance = 0.0f32;
        let mut u = 0.0f32;
        let mut v = 0.0f32;

        let result = triangle_intersect(&ray, triangle, &mut distance, &mut u, &mut v);
        assert_eq!(distance, 5.0);
    }
}