#![feature(test)]
#![feature(in_band_lifetimes)]
#![feature(drain_filter)]

extern crate sdl2;
extern crate gl;
extern crate image;

pub mod render_gl;
pub mod gl_texture;
pub mod window;
mod frame_interpolator;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::gl_texture::GlTexture;
use crate::scene::{SceneEntity, Scene};
use crate::content::wavefront_model_loader::{WaveFrontObjectLoader};
use std::sync::Arc;
use crate::content::store::ModelStore;
use crate::scene::sphere_entity::SphereEntity;
use std::fs::File;
use crate::render_configuration::parser::parse;
use crate::frame_interpolator::FrameInterpolator;
use crate::camera::Camera;
use crate::renderer::{render};
use rand::SeedableRng;
use crate::content::material_builder::MaterialBuilder;
use crate::scene::transform_builder::TransformBuilder;
use crate::scene::plane_entity::PlaneEntity;
use crate::core::plane::Plane;

mod content;
mod renderer;
mod scene;
mod core;
mod render_configuration;
mod frame_processor;
mod camera;
mod color;

fn next_id(id: &mut u32) -> u32 {
    *id = *id + 1;
    *id
}

fn main() {
    let mut rng = rand::rngs::StdRng::seed_from_u64(123);
    let sdl = sdl2::init().unwrap();
    let window = window::Window::create(&sdl).unwrap();

    let loader = WaveFrontObjectLoader {};
    let mut _store = ModelStore::new(Box::new(loader));
    // let args = std::env::args();
    // Input path should be taken from args
    //let mut config = ConfigurationParser{}.parse("/Users/emil/code/rust-rt/src/test.json");
    // let keyframe = &b.keyframes()[0];
    // let updates = &keyframe.updates()[0];

    let f = File::open("/Users/emil/code/rust-rt/assets/scene.json").unwrap();

    let config = parse(f).unwrap();
    let interpolator = FrameInterpolator::new(&config.keyframes);

    let number_of_frames = (config.duration * config.frames_per_second as f64) as usize;
    let seconds_per_frame = 1.0 / config.frames_per_second as f64;
    let samples_per_frame = glm::floor(seconds_per_frame / config.shutter_speed) as usize;

    /*let mut apricot1 = ModelEntity::new(
        store.load("apricot", "/Users/emil/code/rust-rt/assets/models/apricot/Apricot_02_hi_poly.obj")
    );
    let mut apricot2 = ModelEntity::new(
        store.load("apricot", "/Users/emil/code/rust-rt/assets/models/apricot/Apricot_02_hi_poly.obj")
    );

    let mut light_model = store.load("box", "/Users/emil/code/rust-rt/assets/models/crate/crate1.obj");
    light_model.material_overrides().insert("crate1".to_string(), emissive);
    let mut light = ModelEntity::new(
        light_model
    );*/

    /*apricot1.set_position(glm::vec3(-5.0, 0.0, 0.0));
    apricot2.set_position(glm::vec3(5.0, 0.0, 0.0));
    apricot2.set_rotation(glm::vec3(0.0, PI, 0.0));
    apricot2.set_scale(glm::vec3(0.5, 0.5, 0.5));

    light.set_position(glm::vec3(0.0, 5.0, 0.0));
    light.set_scale(glm::vec3(0.01, 0.01, 0.01));*/


    let mut angle = 3.1415 + 0.8;
    let mut bob = 0.0f32;
    let mut event_pump = sdl.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        let mut id = 0;
        let entities: Vec<Box<dyn SceneEntity + Sync + Send>> = vec![
            // Floor
            Box::new(PlaneEntity::new(
                next_id(&mut id),
                Plane::new(glm::vec3(0.0, 0.0, 0.0), glm::vec3(0.0, 1.0, 0.0)),
                10.0,
                MaterialBuilder::new()
                    .with_diffuse_color(glm::vec3(1.0, 1.0, 1.0))
                    .build(),
                TransformBuilder::new()
                    .build(),
            )),

            // Diffuse ball
            Box::new(SphereEntity::new(
                next_id(&mut id),
                1.0,
                MaterialBuilder::new()
                    .with_diffuse_color(glm::vec3(0.5, 0.5, 1.0))
                    // .with_transparency(1.69)
                    // .with_reflectivity(1.0)
                    .build(),
                TransformBuilder::new()
                    .with_translation(glm::vec3(glm::sin(angle) * 3.0, 3.0 + glm::sin(bob), glm::cos(angle) * 3.0))
                    .build(),
            )),

            // Diffuse ball
            Box::new(SphereEntity::new(
                next_id(&mut id),
                1.0,
                MaterialBuilder::new()
                    .with_diffuse_color(glm::vec3(0.5, 0.5, 1.0))
                    .with_emissive_color(glm::vec3(1.0, 1.0, 1.0))
                    .build(),
                TransformBuilder::new()
                    .with_translation(glm::vec3(0.0, 3.0, 0.0))
                    .build(),
            )),
        ];
        let scene2: Arc<dyn Scene + Sync + Send> = Arc::new(scene::octree_scene::Octree::create(entities, 4));

        let mut camera = Camera::new();
        let camera_position = glm::vec3(0.0, 2.5, -15.0) * 1.0;
        let camera_target = glm::vec3(0.0, 0.0, 0.0);
        camera.set_position(camera_position);
        camera.set_direction(glm::normalize(camera_target - camera_position));
        camera.set_resolution(glm::Vector2::new(window.width(), window.height()));

        let mut pixels = vec![0u8; (window.width() * window.height() * 3) as usize];
        let texture = GlTexture::from_pixels(window.width(), window.height(), &pixels).unwrap();
        texture.bind();

        camera.update();
        let image = render(
            &scene2,
            &camera,
            &glm::Vector2::<u32>::new(window.width(), window.height()),
            2,
            &mut rng);


        window.clear();
        texture.set_pixels(window.width(), window.height(), &image.pixels_u8());
        texture.bind();
        window.render();
        window.swap();

        println!("angle {}", angle);

        //angle += 0.1;
        bob += 0.1;
    }
}