pub struct Vao {
    handle: u32
}

impl Vao {
    pub fn create() -> Self {
        let mut handle = 0;
        unsafe { 
            gl::GenVertexArrays(1, &mut handle);
        }

        Self { handle }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.handle);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
