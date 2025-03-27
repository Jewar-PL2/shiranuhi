use super::bios::{self, Bios};

pub struct Bus {
    bios: Bios
}

impl Bus {
    pub fn new(bios: Bios) -> Self {
        Self { bios }
    }

    pub fn load32(&self, address: u32) -> u32 {        
        0
    }
}