#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    // unused, 0
    pub z: u8,
}

#[repr(C)]
pub struct FrameContext {
    pub width: usize,
    pub height: usize,
    pub pixels: *mut Pixel,
    pub ticks: usize,
}

impl FrameContext {
    pub fn pixels(&mut self) -> &mut [Pixel] {
        unsafe { std::slice::from_raw_parts_mut(self.pixels, self.width * self.height) }
    }
}
