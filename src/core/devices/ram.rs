pub struct Ram {
    // TODO: Maybe replace with Vec<u8> instead
    data: Box<[u8; 2 * 1024 * 1024]>
}

impl Ram {
    pub fn new() -> Self {
        // TODO: Make this a lot better
        let data = vec![0xCF; 2 * 1024 * 1024]
            .into_boxed_slice()
            .try_into()
            .unwrap();

        Self { data }
    }

    pub fn load32(&self, address: u32) -> u32 {
        let address = address & 0x1FFFFF;

        let b0 = self.load8(address) as u32;
        let b1 = self.load8(address.wrapping_add(1)) as u32;
        let b2 = self.load8(address.wrapping_add(2)) as u32;
        let b3 = self.load8(address.wrapping_add(3)) as u32;

        (b3 << 24) | (b2 << 16) | (b1 << 8) | b0
    }

    pub fn load16(&self, address: u32) -> u16 {
        let address = address & 0x1FFFFF;

        let b0 = self.load8(address) as u16;
        let b1 = self.load8(address.wrapping_add(1)) as u16;

        (b1 << 8) | b0
    }

    pub fn load8(&self, address: u32) -> u8 {
        let address = address & 0x1FFFFF;

        self.data[address as usize]
    }

    pub fn store32(&mut self, address: u32, value: u32) {
        let address = address & 0x1FFFFF;

        self.store8(address, value as u8);
        self.store8(address.wrapping_add(1), (value >> 8) as u8);
        self.store8(address.wrapping_add(2), (value >> 16) as u8);
        self.store8(address.wrapping_add(3), (value >> 24) as u8);
    }

    pub fn store16(&mut self, address: u32, value: u16) {
        let address = address & 0x1FFFFF;

        self.store8(address, value as u8);
        self.store8(address.wrapping_add(1), (value >> 8) as u8);
    }

    pub fn store8(&mut self, address: u32, value: u8) {
        let address = address & 0x1FFFFF;
        self.data[address as usize] = value;
    }
}