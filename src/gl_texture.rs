use gl;
use std::os::raw::c_void;

pub struct GlTexture {
    id: gl::types::GLuint,
}

impl GlTexture {
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id)
        }
    }

    pub fn from_pixels(width: u32, height: u32, pixels: &Vec<u8>) -> Result<GlTexture, String> {
        unsafe {
            let mut texture: gl::types::GLuint = 0;
            gl::GenTextures(1, &mut texture);

            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const c_void
            );

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST as i32,
            );

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST as i32,
            );

            gl::BindTexture(gl::TEXTURE_2D, 0);

            Ok(GlTexture { id: texture })
        }
    }

    pub fn set_pixels(&self, width: u32, height: u32, pixels: &Vec<u8>) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const c_void
            );

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }


}