pub struct Texture {
    gl_texture: u32,
}

impl Texture {
    pub fn new(width: u32, height: u32) -> Self {
        let mut gl_texture = 0;
        
        unsafe {
            gl::GenTextures(1, &mut gl_texture as _);
            gl::BindTexture(gl::TEXTURE_2D, gl_texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
            gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGB8, width as i32, height as i32);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Texture {
            gl_texture,
        }
    }

    pub fn destroy(&self) {
        unsafe {
            gl::DeleteTextures(1, &self.gl_texture as _);
        }
    }

    pub fn upload_buffer(&self, width: u32, height: u32, data: &[u8]) {
        self.bind();
        
        unsafe {
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                width as i32,
                height as i32,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as _
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.gl_texture);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.destroy()
    }
}
