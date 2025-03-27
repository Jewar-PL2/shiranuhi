use std::{fs::File, io::Read, path::Path};

pub struct Bios {
    data: Box<[u8; 512 * 1024]>
}

impl Bios {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let mut buffer = vec![0x00; 512 * 1024];

        file.read_exact(&mut buffer)?;

        let data = buffer
            .into_boxed_slice()
            .try_into()
            .unwrap();

        Ok(Self { data })
    }

    pub fn load32(&self, address: u32) -> u32 {
        let b0 = self.load8(address) as u32;
        let b1 = self.load8(address.wrapping_add(1)) as u32;
        let b2 = self.load8(address.wrapping_add(2)) as u32;
        let b3 = self.load8(address.wrapping_add(3)) as u32;

        (b3 << 24) | (b2 << 16) | (b1 << 8) | b0
    }

    pub fn load16(&self, address: u32) -> u16 {
        let b0 = self.load8(address) as u16;
        let b1 = self.load8(address.wrapping_add(1)) as u16;

        (b1 << 8) | b0
    }

    pub fn load8(&self, address: u32) -> u8 {
        self.data[address as usize]
    }
}