use sdl2::video::GLContext;
use crate::render_gl;
use crate::render_gl::Program;

pub struct Window {
    window: sdl2::video::Window,
    _context: GLContext,
    vertex_array_object: gl::types::GLuint,
    vertex_buffer_object: gl::types::GLuint,
    shader_program: Program,
}

impl Window {
    pub fn create(sdl: &sdl2::Sdl) -> Result<Window, String> {
        let video = sdl.video().unwrap();

        let sdl_window2 = video
            .window("test", 512, 512)
            .opengl()
            .position_centered()
            .build();

        let mut sdl_window = match sdl_window2 {
            Err(e) => Err(e.to_string()),
            Ok(w) => Ok(w)
        }?;

        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let gl_context = sdl_window.gl_create_context()?;
        let _gl = gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

        unsafe {
            gl::Viewport(0, 0, 512, 512);
            gl::ClearColor(1.0, 0.0, 1.0, 1.0);
        }

        sdl_window.set_size(512, 512).unwrap();

        use std::ffi::CString;
        let vert_shader = render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();
        let frag_shader = render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

        let shader_program = render_gl::Program::from_shaders(
            &[vert_shader, frag_shader]
        ).unwrap();


        // Set up full screen quad
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

        let result = Window {
            window: sdl_window,
            _context: gl_context,
            vertex_array_object: vao,
            vertex_buffer_object: vbo,
            shader_program,
        };

        Ok(result)
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.window.set_size(width, height).unwrap();

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::ClearColor(1.0, 0.0, 1.0, 1.0);
        }
    }

    pub fn width(&self) -> u32 { self.window.size().0 }
    pub fn height(&self) -> u32 { self.window.size().1 }

    pub fn swap(&self) {
        self.window.gl_swap_window();
    }
    pub fn render(&self) {

        self.shader_program.set_used();

        unsafe {
            gl::BindVertexArray(self.vertex_array_object);
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                6,
            );
        }
    }
    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::FrontFace(gl::CW);
        }
    }
}