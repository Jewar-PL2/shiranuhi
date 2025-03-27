use shiranuhi::core::{bios::Bios, bus::Bus, cpu::Cpu};
use spdlog::prelude::*;

fn main() {
    spdlog::default_logger().set_level_filter(LevelFilter::All);

    info!("spdlog works fine");

    let first_lui_instruction = 0x3C080013;

    let bios = Bios::new("SCPH1001.BIN").unwrap();
    assert_eq!(bios.load32(0x00000000), first_lui_instruction);

    let bus = Bus::new(bios);
    assert_eq!(bus.load32(0xBFC00000), first_lui_instruction);

    let mut cpu = Cpu::new(bus);
    loop {
        cpu.clock();
    }
}
