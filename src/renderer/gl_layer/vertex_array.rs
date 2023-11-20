pub struct VertexArray {
    vao: u32,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut vao = 0;
        
        unsafe {
            gl::GenVertexArrays(1, &mut vao as _);
        }

        VertexArray {
            vao
        }
    }

    pub fn destroy(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao as _);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        self.destroy()
    }
}
