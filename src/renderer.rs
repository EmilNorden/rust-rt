use crate::scene::{Scene, ThreadSafeScene};
use crate::camera::Camera;
use crate::core::{Ray, Intersection};
use rand::rngs::StdRng;
use crate::core::math::lerp;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::color::Color;
use std::thread::{Thread, JoinHandle};
use std::thread;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::time::Instant;

pub struct ImageBuffer {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

#[derive(Clone)]
pub struct ScanlineProducer {
    scanline: Arc<AtomicUsize>,
    total_scanlines: usize,
}

impl ScanlineProducer {
    pub fn new(total_scanlines: usize) -> Self {
        ScanlineProducer {
            scanline: Arc::new(AtomicUsize::new(0)),
            total_scanlines,
        }
    }

    pub fn next(&self) -> Option<usize> {
        let next = self.scanline.fetch_add(1, Ordering::SeqCst);

        if next < self.total_scanlines {
            Some(next)
        } else {
            None
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

fn shade(scene: &Arc<dyn Scene+Sync+Send>, ray: &Ray, depth_limit: u32) -> glm::Vec3 {
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
                    direction: reflected_dir,
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
                    direction: glm::normalize(*light.transform().translation() - new_origin),
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

fn render_sample_thread(scene: Arc<dyn Scene+Sync+Send>, camera: Camera, render_width: usize, scanline_producer: ScanlineProducer, tx: Sender<(usize, Vec<Color>)>) -> JoinHandle<()> {
    let scene_clone = scene.clone();
    thread::spawn(move || {

        let mut scanline = Vec::with_capacity(render_width);

        loop {
            match scanline_producer.next() {
                None => break,
                Some(scanline_number) => {
                    for x in 0..render_width {
                        let r = camera.cast_ray(x as usize, scanline_number);

                        let color = shade(&scene_clone, &r, 2);

                        // scanline[x] = Color::from_vec3(&(color * sample_importance));
                        scanline[x] = Color::from_vec3(&color);
                        //image.add_pixel(x as usize, y as usize, Color::from_vec3(&(color * sample_importance)));
                    }

                    tx.send((scanline_number, scanline.to_vec())).unwrap();
                }
            }
        }
    })
}

fn render_sample(scene: &Arc<dyn Scene+Sync+Send>, camera: &Camera, resolution: &glm::Vector2<u32>, rng: &mut StdRng, image: &mut ImageBuffer, sample_importance: f32) {
    let sp = ScanlineProducer::new(resolution.y as usize);
/*
    // let (tx, rx) = std::sync::mpsc::channel();
    let mut threads = Vec::new();
    for tid in 0..3 {
        let sp_clone = sp.clone();
        let scene_clone = scene.clone();
        let camera_clone = camera.clone();
        let render_width = resolution.x;
        threads.push(thread::spawn(move || {}));
    }

    for thread in threads {
        thread.join().unwrap();
    }*/


    let now = Instant::now();
    for y in 0..resolution.y {
        for x in 0..resolution.x {
            let r = camera.cast_ray(x as usize, y as usize);

            let color = shade(scene, &r, 2);

            image.add_pixel(x as usize, y as usize, Color::from_vec3(&(color * sample_importance)));
        }
    }

    println!("Render time: {}ms", now.elapsed().as_millis());
}

pub fn render(scene: &Arc<dyn Scene+Sync+Send>, camera: &Camera, resolution: &glm::Vector2<u32>, rng: &mut StdRng) -> ImageBuffer {
    let mut image = ImageBuffer::new(resolution.x as usize, resolution.y as usize);

    let nsamples = 1;
    for sample in 0..nsamples {
        println!("sample {} of {}", sample + 1, nsamples);
        render_sample(scene, camera, resolution, rng, &mut image, 1.0 / nsamples as f32);
    }


    image
}


