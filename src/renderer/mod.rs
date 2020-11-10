use glm::{Vector3 as Vec3, Vector2 as Vec2};
use crate::core::Ray;

struct ImagePlane {
    u: Vec3<f32>,
    v: Vec3<f32>,
    origin: Vec3<f32>,

    // Do I need these 3?
    d: f32,
    pixel_width: f32,
    pixel_height: f32,
}

pub struct Camera {
    image_plane: ImagePlane,
    position: Vec3<f32>,
    direction: Vec3<f32>,
    up: Vec3<f32>,
    resolution: Vec2<u32>,
    aspect_ratio: f32,
    fov: f32,
    rebuild_image_plane: bool,
}


impl ImagePlane {
    pub fn build(&mut self, field_of_view: f64, aspect_ratio: f64, position: Vec3<f32>, direction: Vec3<f32>, up: Vec3<f32>, resolution: Vec2<u32>) {
        const DISTANCE: f32 = 10.0;

        let width = 2.0 * DISTANCE as f64 * (field_of_view / 2.0).tan();
        let height = width * aspect_ratio as f64;

        let n: Vec3<f32> = glm::normalize(direction * -1.0f32);

        self.u = glm::normalize(glm::cross(up, n));
        self.v = glm::normalize(glm::cross(n, self.u));

        let image_plane_center: Vec3<f32> = position - (n * DISTANCE);
        self.d = glm::length(image_plane_center);

        self.origin = image_plane_center +
            (self.u * (width / 2.0) as f32) -
            (self.v * (height / 2.0) as f32);

        self.pixel_width = (width / resolution.x as f64) as f32;
        self.pixel_height = (height / resolution.y as f64) as f32;
    }

    pub fn point_on_plane(&self, x: usize, y: usize) -> glm::Vector3<f32> {
        self.origin - (self.u * self.pixel_width * (x as f32)) + (self.v * self.pixel_height * (y as f32))
    }
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: glm::Vector3::new(0.0, 0.0, 0.0),
            direction: glm::Vector3::new(0.0, 0.0, -1.0),
            up: glm::Vector3::new(0.0, 1.0, 0.0),
            resolution: Vec2::new(256, 256),
            aspect_ratio: 1.0,
            fov: 1.22173048f32,
            rebuild_image_plane: true,
            image_plane: ImagePlane {
                origin: Vec3::new(0.0, 0.0, 0.0),
                u: Vec3::new(0.0, 0.0, 0.0),
                v: Vec3::new(0.0, 0.0, 0.0),
                d: 0.0,
                pixel_width: 0.0,
                pixel_height: 0.0,
            },
        }
    }

    pub fn set_position(&mut self, pos: Vec3<f32>) {
        self.position = pos;
        self.rebuild_image_plane = true;
    }

    pub fn set_direction(&mut self, dir: Vec3<f32>) {
        self.direction = glm::normalize(dir);
        self.rebuild_image_plane = true;
    }

    #[allow(dead_code)]
    pub fn set_up(&mut self, up: Vec3<f32>) {
        self.up = up;
        self.rebuild_image_plane = true;
    }

    #[allow(dead_code)]
    pub fn set_field_of_view(&mut self, value: f32) {
        self.fov = value;
        self.rebuild_image_plane = true;
    }

    pub fn set_resolution(&mut self, res: Vec2<u32>) {
        self.resolution = res;
        self.aspect_ratio = res.x as f32 / res.y as f32;
        self.rebuild_image_plane = true;
    }

    pub fn update(&mut self) {
        if self.rebuild_image_plane {
            self.rebuild_image_plane = false;
            self.image_plane.build(self.fov as f64, self.aspect_ratio as f64, self.position, self.direction, self.up, self.resolution);
        }
    }

    pub fn cast_ray(&mut self, x: usize, y: usize) -> Ray {
        self.update();
        Ray {
            origin: self.position,
            direction: glm::normalize(self.image_plane.point_on_plane(x, y) - self.position)
        }
    }
}
