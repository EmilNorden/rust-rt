use super::*;
use std::fmt::Display;
use serde::export::Formatter;

#[derive(Clone)]
pub struct AABB {
    pub min: glm::Vector3<f32>,
    pub max: glm::Vector3<f32>,
}

impl Display for AABB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{},{}] - [{},{},{}]", self.min.x, self.min.y, self.min.z, self.max.x, self.max.y, self.max.z)
    }
}

impl AABB {
    pub fn from_bounds<I>(bounds: &I) -> AABB where I: IntoIterator<Item=AABB> + Clone {
        let bounds_clone = (*bounds).clone();

        let mut result = AABB {
            min: glm::Vector3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX),
            max: glm::Vector3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN),
        };

        for x in bounds_clone.into_iter() {
            result.expand(&x);
        }

        result
    }

    pub fn intersects_bounds(&self, other: &AABB) -> bool {
        //TODO: WRITE TESTS
        for dimension in 0..3 {
            if self.max[dimension] < other.min[dimension] {
                return false;
            }
            if self.min[dimension] > other.max[dimension] {
                return false;
            }
        }

        true
    }

    pub fn contains(&self, point: &glm::Vector3<f32>) -> bool {
        point.x >= self.min.x && point.x < self.max.x &&
            point.y >= self.min.y && point.y < self.max.y &&
            point.z >= self.min.z && point.z < self.max.z
    }

    pub fn from_location_and_size(location: &glm::Vector3<f32>, size: &glm::Vector3<f32>) -> AABB {
        //TODO: WRITE TESTS
        AABB { min: location.clone(), max: *location + *size }
    }

    pub fn size(&self) -> glm::Vector3<f32> {
        //TODO: WRITE TESTS
        self.max - self.min
    }

    pub fn combine(first: &AABB, second: &AABB) -> AABB{
        let mut result = first.clone();
        result.expand(second);

        result
    }

    pub fn expand(&mut self, other: &AABB) {
        //TODO: WRITE TESTS
        self.min.x = self.min.x.min(other.min.x);
        self.min.y = self.min.y.min(other.min.y);
        self.min.z = self.min.z.min(other.min.z);

        self.max.x = self.max.x.max(other.max.x);
        self.max.y = self.max.y.max(other.max.y);
        self.max.z = self.max.z.max(other.max.z);
    }

    pub fn from_vector3<I>(vertices: &I) -> AABB where I: IntoIterator<Item=glm::Vector3<f32>> + Clone {
        let vertices_clone = (*vertices).clone();
        let mut smallest = glm::Vector3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX);
        let mut largest = glm::Vector3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN);

        let mut is_empty = true;
        for vertex in vertices_clone.into_iter() {
            is_empty = false;
            for index in 0..3usize {
                if vertex[index] < smallest[index] {
                    smallest[index] = vertex[index];
                }

                if vertex[index] > largest[index] {
                    largest[index] = vertex[index];
                }
            }
        }

        assert!(!is_empty, "AABB::from_vector3 called with empty iterator");


        AABB {
            min: smallest,
            max: largest,
        }
    }

    pub fn transform(&self, mat: &glm::Mat4) -> AABB {
        let corners = vec![
            (*mat * glm::Vector4 { x: self.min.x, y: self.min.y, z: self.min.z, w: 1.0 }).truncate(3),
            (*mat * glm::Vector4 { x: self.min.x, y: self.min.y, z: self.max.z, w: 1.0 }).truncate(3),
            (*mat * glm::Vector4 { x: self.min.x, y: self.max.y, z: self.min.z, w: 1.0 }).truncate(3),
            (*mat * glm::Vector4 { x: self.max.x, y: self.min.y, z: self.min.z, w: 1.0 }).truncate(3),
            (*mat * glm::Vector4 { x: self.max.x, y: self.max.y, z: self.max.z, w: 1.0 }).truncate(3),
            (*mat * glm::Vector4 { x: self.max.x, y: self.max.y, z: self.min.z, w: 1.0 }).truncate(3),
            (*mat * glm::Vector4 { x: self.max.x, y: self.min.y, z: self.max.z, w: 1.0 }).truncate(3),
            (*mat * glm::Vector4 { x: self.min.x, y: self.max.y, z: self.max.z, w: 1.0 }).truncate(3)
        ];


        AABB::from_vector3(&corners)
    }
}

pub fn ray_aabb_intersect(ray: &Ray, aabb: &AABB) -> bool {
    const EPSILON: f32 = 9.99999997475243E-07;

    let mut near = std::f32::MIN_POSITIVE;
    let mut far = std::f32::MAX;

    for dimension in 0..3 {
        if ray.direction[dimension].abs() < EPSILON {
            if ray.origin[dimension] < aabb.min[dimension] || ray.origin[dimension] > aabb.max[dimension] {
                return false;
            }
        } else {
            let mut t1 = (aabb.min[dimension] - ray.origin[dimension]) / ray.direction[dimension];
            let mut t2 = (aabb.max[dimension] - ray.origin[dimension]) / ray.direction[dimension];
            if t1 > t2 {
                let temp = t1;
                t1 = t2;
                t2 = temp;
            }

            if t1 > near {
                near = t1;
            }

            if t2 < far {
                far = t2;
            }

            if near > far || far < 0.0 {
                return false;
            }
        }
    }

    true
}

pub fn ray_triangle_intersect(ray: &Ray, triangle: (glm::Vector3<f32>, glm::Vector3<f32>, glm::Vector3<f32>), distance: &mut f32, result_u: &mut f32, result_v: &mut f32) -> bool {
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
    use num_traits::identities::One;

    fn nearly_equal(a: f32, b: f32) -> bool {
        let abs_a = a.abs();
        let abs_b = b.abs();
        let diff = (a - b).abs();

        if a == b { // Handle infinities.
            true
        } else if a == 0.0 || b == 0.0 || diff < std::f32::MIN_POSITIVE {
            // One of a or b is zero (or both are extremely close to it,) use absolute error.
            diff < (std::f32::EPSILON * std::f32::MIN_POSITIVE)
        } else { // Use relative error.
            (diff / f32::min(abs_a + abs_b, std::f32::MAX)) < std::f32::EPSILON
        }
    }

    fn assert_identical_vectors(v1: &glm::Vector3<f32>, v2: &glm::Vector3<f32>) {
        assert!(nearly_equal(v1.x, v2.x), "Vectors are not equal: [{}, {}, {}] and [{}, {}, {}]", v1.x, v1.y, v1.z, v2.x, v2.y, v2.z);
        assert!(nearly_equal(v1.y, v2.y), "Vectors are not equal: [{}, {}, {}] and [{}, {}, {}]", v1.x, v1.y, v1.z, v2.x, v2.y, v2.z);
        assert!(nearly_equal(v1.z, v2.z), "Vectors are not equal: [{}, {}, {}] and [{}, {}, {}]", v1.x, v1.y, v1.z, v2.x, v2.y, v2.z);
        // assert_eq!(glm::is_approx_eq(v1, v2), true, "Vectors are not equal: [{}, {}, {}] and [{}, {}, {}]", v1.x, v1.y, v1.z, v2.x, v2.y, v2.z);
    }

    #[test]
    fn aabb_from_vector3_should_return_correct_bounding_box() {
        let input = vec![
            glm::Vector3::new(2.0, 3.0, 4.0),
            glm::Vector3::new(2.0, 1000.0, 4.0),
            glm::Vector3::new(0.0, 999.0, 22.0),
            glm::Vector3::new(2.5, -0.5, 4.0),
            glm::Vector3::new(2.0, -0.5, -1000.0),
        ];

        let result = AABB::from_vector3(&input);

        assert_identical_vectors(&result.min, &glm::Vector3::new(0.0, -0.5, -1000.0));
        assert_identical_vectors(&result.max, &glm::Vector3::new(2.5, 1000.0, 22.0));
    }

    #[test]
    fn aabb_from_vector3_should_handle_single_vector() {
        let input = vec![
            glm::Vector3::new(2.0, 3.0, 4.0),
        ];

        let result = AABB::from_vector3(&input);

        assert_identical_vectors(&result.min, &input[0]);
        assert_identical_vectors(&result.max, &input[0]);
    }

    #[test]
    #[should_panic(expected = "AABB::from_vector3 called with empty iterator")]
    fn aabb_from_vector3_should_panic_on_empty_input() {
        let input = Vec::new();

        let _result = AABB::from_vector3(&input);
    }

    #[test]
    fn aabb_transform_should_handle_identity() {
        let aabb = AABB {
            min: glm::Vector3::new(0.0, 0.0, 0.0),
            max: glm::Vector3::new(10.0, 10.0, 10.0),
        };

        let result = aabb.transform(&glm::Matrix4::<f32>::one());
        assert_identical_vectors(&result.min, &glm::Vector3::new(0.0, 0.0, 0.0));
        assert_identical_vectors(&result.max, &glm::Vector3::new(10.0, 10.0, 10.0));
    }

    #[test]
    fn aabb_transform_should_handle_composed_transform() {
        let aabb = AABB {
            min: glm::Vector3::new(-10.0, -10.0, -20.0),
            max: glm::Vector3::new(10.0, 10.0, 20.0),
        };
        let result = aabb.transform(&glm::ext::scale(&glm::ext::rotate::<f32>(&glm::Matrix4::<f32>::one(), 1.57079633, glm::Vector3::new(0.0, 1.0, 0.0)), glm::Vector3::new(2.0, 2.0, 2.0)));

        assert_identical_vectors(&result.min, &glm::Vector3::new(-40.0, -20.0, -20.0));
        assert_identical_vectors(&result.max, &glm::Vector3::new(40.0, 20.0, 20.0));
    }

    #[test]
    fn triangle_intersect_should_handle_simple_intersection() {
        let ray = Ray {
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

        let result = ray_triangle_intersect(&ray, triangle, &mut distance, &mut u, &mut v);
        assert_eq!(result, true);
    }

    #[test]
    fn triangle_intersect_should_handle_backface_intersection() {
        let ray = Ray {
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

        let result = ray_triangle_intersect(&ray, triangle, &mut distance, &mut u, &mut v);
        assert_eq!(result, true);
    }

    #[test]
    fn triangle_intersect_should_handle_simple_miss() {
        let ray = Ray {
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

        let result = ray_triangle_intersect(&ray, triangle, &mut distance, &mut u, &mut v);
        assert_eq!(result, false);
    }

    #[test]
    fn triangle_intersect_should_handle_skewed_triangle() {
        let ray = Ray {
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

        let result = ray_triangle_intersect(&ray, triangle, &mut distance, &mut u, &mut v);
        assert_eq!(result, true);
    }

    #[test]
    fn triangle_intersect_should_handle_parallel_triangle_miss() {
        let ray = Ray {
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

        let result = ray_triangle_intersect(&ray, triangle, &mut distance, &mut u, &mut v);
        assert_eq!(result, false);
    }

    #[test]
    fn triangle_intersect_should_return_correct_distance() {
        let ray = Ray {
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

        let _ = ray_triangle_intersect(&ray, triangle, &mut distance, &mut u, &mut v);
        assert_eq!(distance, 5.0);
    }
}