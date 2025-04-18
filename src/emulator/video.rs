pub struct Video;

impl Video {
    pub fn new() -> Self {
        Self
    }

    pub fn render_framebuffer(&self) -> Vec<u8> {
        vec![0; 320 * 224 * 4]
    }
}
