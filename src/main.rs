#![feature(test)]

extern crate sdl2;
extern crate gl;

pub mod render_gl;
pub mod texture;
pub mod window;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::texture::Texture;
use crate::scene::SceneEntity;
use num_traits::identities::One;

mod content;
mod renderer;
mod scene;
mod core;

pub fn func_borrowing<I>(input: &I) -> f32 where I : IntoIterator<Item = f32> + Clone
{
    let foo = (*input).clone();
    for x in foo.into_iter() {
        if x > 2.0 {
            return x;
        }
    }

    return 0.0;
}

fn main() {
    // let foo2 = func_borrowing(values.into_iter());
    //let foo2 = func_borrowing(&values2);


    let _foo = content::load("/Users/emil/code/rust-rt/assets/models/apricot/Apricot_02_hi_poly.obj").unwrap();
    // let _foo = content::load("/Users/emil/code/rust-rt/assets/models/crate/crate1.obj").unwrap();
    // let _foo = content::load("/Users/emil/code/rust-rt/assets/models/horse/horse.obj").unwrap();
    // let mut identity = glm::ext::rotate(&glm::Matrix4::<f32>::one(), 90.0f32.to_radians(), glm::Vector3::new(0.0, 1.0, 0.0));
    let identity = glm::Matrix4::<f32>::one();
    /*let entity = SceneEntity {
        mesh: &_foo.meshes[0],
        inverse_transform: identity, // TODO: SHOULD INVERSE
    };*/
    let entity = SceneEntity::new(&_foo.meshes[0], glm::inverse(&identity));

    let scene = scene::create_scene();
    // scene.add(entity);

    let entities = vec![entity];
    let scene2 = scene::octree_scene::Octree::create(&entities);



    let sdl = sdl2::init().unwrap();
    let window = window::Window::create(&sdl).unwrap();

    let mut camera = renderer::Camera::new();
    camera.set_position(glm::Vector3::new(0.0, 2.5, 15.0));
    camera.set_direction(glm::Vector3::new(0.0, 0.0, -1.0));
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
                Some(intersection) => glm::Vector3::new(1.0, 0.0, 0.0),
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

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::FrontFace(gl::CW);
        }


        texture.set_pixels(window.width(), window.height(), &pixels);
        texture.bind();

        /*unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                512,
                512,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const c_void
            );
        }*/

        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                6,
            );
        }

        // window.gl_swap_window();
        window.swap();
    }


    // let importer = assimp::Importer::new();
    println!("Hello, world!77");
}