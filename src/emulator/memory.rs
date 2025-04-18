pub struct Memory {
    ram: [u8; 0x10000],
}

impl Memory {
    pub fn new() -> Self {
        Self { ram: [0; 0x10000] }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }
}
