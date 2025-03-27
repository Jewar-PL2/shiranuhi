pub mod cpu;
pub mod bus;
pub mod bios;

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