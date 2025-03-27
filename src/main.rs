use shiranuhi::core::bios::Bios;
use spdlog::prelude::*;

fn main() {
    info!("spdlog works fine");

    let bios = Bios::new("SCPH1001.BIN").unwrap();
    assert_eq!(bios.load32(0x00000000), 0x3C080013);
}
