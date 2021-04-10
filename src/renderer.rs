use crate::scene::Scene;
use crate::camera::Camera;
use crate::core::{Ray, Intersection};
use rand::rngs::StdRng;
use std::ops::{Mul, Add};

pub struct ImageBuffer {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn from_vec3(vec: &glm::Vec3) -> Self {
        Color {
            r: (vec.x * 255.0) as u8,
            g: (vec.y * 255.0) as u8,
            b: (vec.z * 255.0) as u8
        }
    }
}

impl ImageBuffer {
    const BYTES_PER_PIXEL: usize = 3;

    pub fn new(width: usize, height: usize) -> Self {
        ImageBuffer {
            pixels: vec![0u8; width * height * ImageBuffer::BYTES_PER_PIXEL],
            width,
            height,
        }
    }

    pub fn pixels(&self) -> &Vec<u8> {
        &self.pixels
    }

    #[inline(always)]
    pub fn add_pixel(&mut self, x: usize, y: usize, color: Color) {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);

        let pixel_offset = y * self.width * ImageBuffer::BYTES_PER_PIXEL + (x * ImageBuffer::BYTES_PER_PIXEL);
        self.pixels[pixel_offset] += color.r;
        self.pixels[pixel_offset + 1] += color.g;
        self.pixels[pixel_offset + 2] += color.b;
    }

    #[inline(always)]
    pub fn pixel(&self, x: usize, y: usize) -> Color {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);

        let pixel_offset = y * self.width * ImageBuffer::BYTES_PER_PIXEL + (x * ImageBuffer::BYTES_PER_PIXEL);

        Color {
            r: self.pixels[pixel_offset],
            g: self.pixels[pixel_offset + 1],
            b: self.pixels[pixel_offset + 2],
        }
    }
}

fn shade(scene: &dyn Scene, ray: &Ray, depth_limit: u32) -> glm::Vec3 {
    if depth_limit == 0 {
        return glm::vec3(0.0, 0.0, 0.0);
    }

    match scene.find_intersection(ray) {
        None => {
            glm::vec3(0.0, 0.0, 0.0)
        }
        Some(intersection) => {

            let coordinate = intersection.coordinate();
            let norm = intersection.world_space_normal();

            // Collect incoming light
            // - own emission
            // - bidirectional

            let mut reflected = glm::vec3(0.0, 0.0, 0.0);
            if intersection.material().reflectivity() > 0.0 {
                let reflected_dir =
                    glm::reflect(ray.direction, intersection.world_space_normal());

                let reflected_ray = Ray {
                    origin: coordinate + (norm * 0.1),
                    direction: reflected_dir
                };

                reflected = shade(scene, &reflected_ray, depth_limit - 1);

            }

            let mut diffuse = intersection.material().sample_diffuse(&intersection.texture_coordinates());
            let mut direct_light = glm::vec3(0.0, 0.0, 0.0);
            for light in scene.get_emissive_entities() {

                if intersection.entity_id() == 1 {
                    let ff = 2323;
                }

                let new_origin = coordinate + (norm * 0.1);
                let shadow_ray = Ray {
                    origin: new_origin,
                    direction: glm::normalize( *light.transform().translation() - new_origin)
                };

                if let Some(light_intersection) = scene.find_intersection(&shadow_ray) {
                    let coo = light_intersection.coordinate();
                    if coordinate.x < 3.0 && intersection.entity_id() == 1 && light_intersection.entity_id() == intersection.entity_id() {
                        let fsdf = 34;
                    }
                    if light_intersection.entity_id() == light.entity_id() {
                        direct_light = direct_light + *light_intersection.material().emission();
                    }

                }
            }
            lerp(
                *intersection.material().emission() + (diffuse * direct_light),
                reflected,
                intersection.material().reflectivity())
       }
    }
}

fn lerp<T>(a: T, b: T, factor: f32) -> T
    where
        T: Mul<f32, Output = T> + Add<T, Output = T>
{
    a * (1.0 - factor) + b * factor
}

fn render_sample(scene: &dyn Scene, camera: &Camera, resolution: &glm::Vector2<u32>, rng: &mut StdRng, image: &mut ImageBuffer, sample_importance: f32) {
    for y in 0..resolution.y {
        for x in 0..resolution.x {
            let r = camera.cast_ray(x as usize, y as usize);

            //let foo = generate_light_path(scene, rng);

            let color = shade(scene, &r, 2);





            //let color = trace_ray(&r, scene, 3);
            image.add_pixel(x as usize, y as usize, Color::from_vec3(&(color * sample_importance)));
        }
    }
}

pub fn render(scene: &dyn Scene, camera: &Camera, resolution: &glm::Vector2<u32>, rng: &mut StdRng) -> ImageBuffer {
    let mut image = ImageBuffer::new(resolution.x as usize, resolution.y as usize);

    let nsamples = 1;
    for sample in 0..nsamples {
        println!("sample {} of {}", sample+1, nsamples);
        render_sample(scene, camera, resolution, rng, &mut image, 1.0 / nsamples as f32);
    }


    image
}

/*fn generate_light_path(scene: &'a dyn Scene, rng: &mut StdRng) -> Vec<Box<dyn Intersection + 'a>> {
    let mut result = Vec::new();

    result.push(scene.get_random_emissive_surface(rng));
    let mut world_ray = Ray {
        origin: result[0].coordinate(),
        direction: result[0].world_space_normal()
    };
    for i in 0..4usize {
        match scene.find_intersection(&world_ray) {
            None => break,
            Some(x) => {
                // Given the incoming ray, surface normal, and the material, what is the outgoing ray?
                // For now, everything is perfect mirrors :)

                let n = x.world_space_normal();
                let outgoing = glm::reflect(world_ray.direction, n);
                world_ray = Ray {
                    origin: x.coordinate(),
                    direction: outgoing
                };
                result.push(x);
            }
        }
    }

    result
}*/

/*fn generate_camera_path(world_ray: &Ray, scene: &dyn Scene) -> Vec<Box<dyn Intersection>> {
    const PATH_LENGTH: usize = 3;
    // let result = Vec::with_capacity(3);

    for _ in 0..PATH_LENGTH {
        scene.find_intersection(world_ray);
    }
    let p = scene.find_intersection(world_ray);


    vec![0u32]
}*/

fn generate_foo() -> [u8; 3] {
    [0, 1, 2]
}

fn trace_ray(world_ray: &Ray, scene: &dyn Scene, remaining_recursions: usize) -> glm::Vec3 {
    match scene.find_intersection(world_ray) {
        None => {
            glm::vec3(0.0, 0.0, 0.0)
        }
        Some(x) => {
            // Collect incoming light
            // - own emission
            // - bidirectional
            x.material().sample_diffuse(&x.texture_coordinates())
        }
    }
}

fn get_emitted_light(intersection: Box<dyn Intersection>, incoming_world_ray: &Ray) -> (glm::Vec3, Ray) {
    panic!("asdsa");
}