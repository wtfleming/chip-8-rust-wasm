use std::io;
use std::io::prelude::*;
use std::fs::File;

use chip_8_lib::cpu::Cpu;

// This file is mostly used for debugging, you shouldn't need to use it
fn main() {
    println!("Starting CPU");

    let mut cpu = Cpu::new();
    cpu.initialize();

    let data = load_game("./chip_8_wasm/static/roms/PONG2").unwrap();
    cpu.load_game(data);

    loop {
        // Emulate one cycle
        cpu.emulate_cycle().unwrap();
    }
}


fn load_game(file_name: &str) -> io::Result<Vec<u8>> {
    println!("load_game() {}", file_name);

    let file_metadata = std::fs::metadata(file_name)?;
    println!("{} is {} bytes in size", file_name, file_metadata.len());

    let mut f = File::open(file_name)?;
    let mut buffer: Vec<u8> = vec![0; file_metadata.len() as usize];
    f.read_exact(&mut buffer)?;

    Ok(buffer)
}
