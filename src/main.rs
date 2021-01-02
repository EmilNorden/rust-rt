#![feature(test)]
#![feature(in_band_lifetimes)]
#![feature(drain_filter)]

extern crate sdl2;
extern crate gl;
extern crate image;

pub mod render_gl;
pub mod texture;
pub mod window;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::texture::Texture;
use crate::scene::SceneEntity;
use num_traits::identities::One;
use crate::content::model_loader::{ModelLoader, LoadOptions};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, BufReader};
use std::cell::RefCell;
use std::sync::Arc;
use crate::render_configuration::{RenderConfiguration, ConfigurationParser};
use crate::content::store::ModelStore;
use crate::frame_processor::FrameProcessor;
use crate::keyframe_interpolator::KeyFrameInterpolator;
use std::f32::consts::PI;
use crate::content::material::Material;

mod content;
mod renderer;
mod scene;
mod core;
mod render_configuration;
mod frame_processor;
mod keyframe_interpolator;


fn main() {
    let loader = ModelLoader{};
    let mut store = ModelStore::new(loader);
    // let args = std::env::args();
    // Input path should be taken from args
    let mut config = ConfigurationParser{}.parse("/Users/emil/code/rust-rt/src/test.json");
    // let keyframe = &b.keyframes()[0];
    // let updates = &keyframe.updates()[0];

    let emissive = Material::new(None, glm::vec3(1.0, 0.0, 0.0));


    let mut apricot1 = SceneEntity::new(
        store.load("apricot", "/Users/emil/code/rust-rt/assets/models/apricot/Apricot_02_hi_poly.obj")
    );
    let mut apricot2 = SceneEntity::new(
        store.load("apricot", "/Users/emil/code/rust-rt/assets/models/apricot/Apricot_02_hi_poly.obj")
    );

    let mut light_model = store.load("box", "/Users/emil/code/rust-rt/assets/models/crate/crate1.obj");
    light_model.material_overrides().insert("crate1".to_string(), emissive);
    let mut light = SceneEntity::new(
        light_model
    );

    apricot1.set_position(glm::vec3(-5.0, 0.0, 0.0));
    apricot2.set_position(glm::vec3(5.0, 0.0, 0.0));
    apricot2.set_rotation(glm::vec3(0.0, PI, 0.0));
    apricot2.set_scale(glm::vec3(0.5, 0.5, 0.5));

    light.set_position(glm::vec3(0.0, 5.0, 0.0));
    light.set_scale(glm::vec3(0.01, 0.01, 0.01));

    let entities = vec![
        apricot1, apricot2, light
    ];

    let scene2 = scene::octree_scene::Octree::create(entities, 4);

    let sdl = sdl2::init().unwrap();
    let window = window::Window::create(&sdl).unwrap();

    let mut camera = renderer::Camera::new();
    camera.set_position(glm::Vector3::new(0.0, 2.5, 15.0));
    // camera.set_position(glm::Vector3::new(0.0, 300.0, 300.0));
    camera.set_direction(glm::Vector3::new(0.0, 0.0, -1.0));
    // camera.set_direction(glm::Vector3::new(0.0, -1.0, -1.0));
    camera.set_resolution(glm::Vector2::new(window.width(), window.height()));


    let mut pixels = vec![0u8; (window.width() * window.height() * 3) as usize]; // Vec::<u8>::with_capacity((window.width() * window.height() * 3) as usize);

    let texture = Texture::from_pixels(window.width(), window.height(), &pixels).unwrap();
    texture.bind();
    let mut event_pump = sdl.event_pump().unwrap();

    for y in 0..window.height() as usize {
        for x in 0..window.width() as usize {
            let ray = camera.cast_ray(x, y);
            let result = scene2.trace(&ray);
            let color: glm::Vector3<f32> = match result {
                Some(intersection) => {
                    let _normal = intersection.mesh.calculate_object_space_normal(&intersection.indices, intersection.u, intersection.v);
                    let texcoords = intersection.mesh.calculate_texcoords(&intersection.indices, intersection.u, intersection.v);
                    match intersection.material {
                        Some(m) => m.sample_diffuse(texcoords.x, texcoords.y),
                        None => glm::vec3(0.0, 0.0, 0.0),
                    }
                },
                None => glm::vec3(0.0, 1.0, 0.0)
            };

            pixels[(y * window.width() as usize * 3) + (x * 3)] = (color.x * 255.0) as u8;
            pixels[(y * window.width() as usize * 3) + (x * 3) + 1] = (color.y * 255.0) as u8;
            pixels[(y * window.width() as usize * 3) + (x * 3) + 2] = (color.z * 255.0) as u8;
        }

        println!("row {}", y);
    }

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

        window.clear();


        texture.set_pixels(window.width(), window.height(), &pixels);
        texture.bind();
        window.render();
        window.swap();
    }
}