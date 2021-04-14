use float_cmp::{ApproxEq, F32Margin};

pub struct Plane {
    origin: glm::Vec3,
    normal: glm::Vec3,
    u: glm::Vec3,
    v: glm::Vec3,
}

impl Plane {
    pub fn new(origin: glm::Vec3, normal: glm::Vec3) -> Self {
        let facing_up = glm::dot(glm::vec3(0.0, 1.0, 0.0), normal)
            .approx_eq(1.0, F32Margin { ulps: 2, epsilon: std::f32::EPSILON });

        let u = glm::normalize(glm::cross(
            if facing_up { glm::vec3(1.0, 0.0, 0.0)} else { glm::vec3(0.0, 1.0, 0.0) },
            normal));
        let v = glm::normalize(glm::cross(u, normal));

        Plane {
            origin,
            normal,
            u,
            v
        }
    }

    pub fn origin(&self) -> glm::Vec3 {
        self.origin
    }

    pub fn normal(&self) -> glm::Vec3 {
        self.normal
    }

    pub fn u(&self) -> glm::Vec3 {
        self.u
    }

    pub fn v(&self) -> glm::Vec3 {
        self.v
    }
}