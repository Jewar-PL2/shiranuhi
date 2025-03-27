use super::bios::Bios;

const BIOS_RANGE: Range = Range(0x1FC00000, 512 * 1024);

pub struct Bus {
    bios: Bios
}

impl Bus {
    pub fn new(bios: Bios) -> Self {
        Self { bios }
    }

    pub fn load32(&self, address: u32) -> u32 {        
        let address = get_masked_address(address);
        
        if let Some(offset) = BIOS_RANGE.contains(address) {
            return self.bios.load32(offset);
        }

        panic!("INVALID ADDRESS: 0x{:08X}", address);
    }
}

pub struct Range(u32, u32);
impl Range {
    pub fn contains(self, address: u32) -> Option<u32> {
        let Range(start, size) = self;

        if address >= start && address < start + size {
            Some(address - start)
        } else {
            None
        }
    }
}

const REGION_MASKS: [u32; 8] = [
    0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, // KUSEG: 2048MB
    0x7FFFFFFF,                                     // KSEG0:  512MB
    0x1FFFFFFF,                                     // KSEG1:  512MB
    0xFFFFFFFF, 0xFFFFFFFF,                         // KSEG2: 1024MB
];

fn get_masked_address(address: u32) -> u32 {
    address & REGION_MASKS[(address >> 29) as usize]
} 