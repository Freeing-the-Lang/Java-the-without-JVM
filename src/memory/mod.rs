pub struct Arena {
    buf: Vec<u8>,
    offset: usize,
}

impl Arena {
    pub fn new(size: usize) -> Self {
        Self { buf: vec![0; size], offset: 0 }
    }

    pub fn alloc(&mut self, size: usize) -> *mut u8 {
        let ptr = unsafe { self.buf.as_mut_ptr().add(self.offset) };
        self.offset += size;
        ptr
    }
}
