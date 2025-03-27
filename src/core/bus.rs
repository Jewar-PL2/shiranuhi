use super::bios::Bios;

pub struct Bus {
    bios: Bios
}

impl Bus {
    pub fn new(bios: Bios) -> Self {
        Self { bios }
    }
}