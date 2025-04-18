pub mod m68k;
pub mod z80;
pub mod memory;
pub mod video;
pub mod audio;
pub mod rom;

use self::{memory::Memory, video::Video, audio::Audio};

pub struct RGen {
    memory: Memory,
    video: Video,
    audio: Audio,
}

impl RGen {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            video: Video::new(),
            audio: Audio::new(),
        }
    }

    pub fn tick_frame(&mut self) {
    }
}
