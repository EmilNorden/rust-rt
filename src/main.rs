#![feature(test)]

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
use crate::content::model_loader::{ModelLoader};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, BufReader};
use std::cell::RefCell;
use std::sync::Arc;
use crate::scene_description::SceneDescription;
use crate::content::store::ModelStore;

mod content;
mod renderer;
mod scene;
mod core;
mod scene_description;
mod frame_processor;


fn main() {
    let loader = ModelLoader{};
    let mut store = ModelStore::new(loader);
    // let args = std::env::args();
    // Input path should be taken from args
    let file = File::open("/Users/emil/code/rust-rt/src/test.json").unwrap();
    let reader = BufReader::new(file);
    let b: SceneDescription = serde_json::from_reader(reader).unwrap();

    let keyframe = &b.keyframes()[0];
    let updates = &keyframe.updates()[0];


    let _foo = store.load("apricot", "/Users/emil/code/rust-rt/assets/models/apricot/Apricot_02_hi_poly.obj");
    let identity = glm::Matrix4::<f32>::one();

    //let _foo = loader.load("/Users/emil/code/rust-rt/assets/models/crate/crate1.obj").unwrap();
    //let identity = glm::ext::scale(&glm::Matrix4::<f32>::one(), glm::Vector3::<f32>::new(0.050, 0.050, 0.050));
    //let identity2 = glm::ext::translate(&glm::Matrix4::<f32>::one(), glm::Vector3::new(-6.0, 0.0, 0.0)) * glm::ext::scale(&glm::Matrix4::<f32>::one(), glm::Vector3::<f32>::new(0.050, 0.050, 0.050));

    // Uncomment for horse
    //let identity = glm::ext::scale(&glm::Matrix4::<f32>::one(), glm::Vector3::<f32>::new(1.0, 1.0, 1.0)) * glm::ext::rotate(&glm::Matrix4::<f32>::one(), 90.0f32.to_radians(), glm::Vector3::new(0.0, 1.0, 0.0));
    /*let entity = SceneEntity {
        mesh: &_foo.meshes[0],
        inverse_transform: identity, // TODO: SHOULD INVERSE
    };*/


    // scene.add(entity);

    let entities = vec![
        SceneEntity::new(&_foo.meshes[0], identity),
    ];
    let scene2 = scene::octree_scene::Octree::create(&entities, 4);

    let sdl = sdl2::init().unwrap();
    let window = window::Window::create(&sdl).unwrap();

    let mut camera = renderer::Camera::new();
    camera.set_position(glm::Vector3::new(0.0, 2.5, 15.0));
    // camera.set_position(glm::Vector3::new(0.0, 300.0, 300.0));
    camera.set_direction(glm::Vector3::new(0.0, 0.0, -1.0));
    // camera.set_direction(glm::Vector3::new(0.0, -1.0, -1.0));
    camera.set_resolution(glm::Vector2::new(window.width(), window.height()));

    use std::ffi::CString;
    let vert_shader = render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();
    let frag_shader = render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

    let shader_program = render_gl::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();

    let vertices: Vec<f32> = vec![
        -1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, // uppe vänster?
        1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, // uppe höger
        1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, // nere höger

        -1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, // uppe vänster?
        1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, // nere höger
        -1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // nere vänster
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(), // offset of the first component
        );

        gl::EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
        gl::VertexAttribPointer(
            1, // index of the generic vertex attribute ("layout (location = 1)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid, // offset of the first component
        );

        gl::EnableVertexAttribArray(2); // this is "layout (location = 2)" in vertex shader
        gl::VertexAttribPointer(
            2, // index of the generic vertex attribute ("layout (location = 1)")
            2, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid, // offset of the first component
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
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
                    let material = &_foo.materials[intersection.material_index];
                    material.sample_diffuse(texcoords.x, texcoords.y)
                },
                None => glm::Vector3::new(0.0, 1.0, 0.0)
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

        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                6,
            );
        }
        window.swap();
    }
}