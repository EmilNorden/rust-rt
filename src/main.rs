extern crate assimp;
extern crate sdl2;
extern crate gl;

pub mod render_gl;
pub mod texture;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::texture::Texture;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("test", 800, 600)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(1.0, 0.0, 1.0, 1.0);
    }

    use std::ffi::CString;
    let vert_shader = render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();
    let frag_shader = render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

    let shader_program = render_gl::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();

    let vertices: Vec<f32> = vec![
        -1.0, -1.0, 0.0,    1.0, 0.0, 0.0,      0.0, 0.0, // nere vänster?
        1.0, -1.0, 0.0,    0.0, 1.0, 0.0,      0.0, 0.0, // nere höger
        -1.0, 1.0, 0.0,    0.0, 0.0, 1.0,      0.0, 0.0, // uppe vänster

        -1.0, 1.0, 0.0,    0.0, 0.0, 1.0,      0.0, 0.0, // uppe vänster
        1.0, -1.0, 0.0,    0.0, 1.0, 0.0,      0.0, 0.0, // nere höger
        1.0, 1.0, 0.0,    0.0, 0.0, 0.0,      0.0, 0.0, // uppe höger
    ];

    /*let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,    1.0, 0.0, 0.0,      0.0, 0.0,
        0.5, -0.5, 0.0,     0.0, 1.0, 0.0,      0.0, 1.0,
        0.0, 0.5, 0.0,      0.0, 0.0, 1.0,      1.0, 0.0
    ];*/

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
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
            std::ptr::null() // offset of the first component
        );

        gl::EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
        gl::VertexAttribPointer(
            1, // index of the generic vertex attribute ("layout (location = 1)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
        );

        gl::EnableVertexAttribArray(2); // this is "layout (location = 2)" in vertex shader
        gl::VertexAttribPointer(
            2, // index of the generic vertex attribute ("layout (location = 1)")
            2, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

    }

    let mut pixels: [u8; 512*512 * 3] = [0; 512*512 * 3];
    let mut color = 0;
    let texture = Texture::from_pixels(512, 512, pixels.to_vec()).unwrap();
    texture.bind();
    let mut i = 0;
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

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::FrontFace(gl::CW);
        }

        pixels[i] = color;
        color = (color + 1) % 255;
        i = (i + 1) % (512*512*3);

        texture.set_pixels(512, 512, pixels.to_vec());
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
                6
            );
        }

        window.gl_swap_window();
    }


    // let importer = assimp::Importer::new();
    println!("Hello, world!77");
}