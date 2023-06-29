pub struct Vbo {
    handle: u32
}

impl Vbo {
    pub fn create() -> Self {
        let mut handle = 0;
        unsafe { 
            gl::GenBuffers(1, &mut handle);
        }

        Self { handle }
    }
}
