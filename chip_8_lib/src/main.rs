use std::io;
use std::io::prelude::*;
use std::fs::File;

use chip_8_lib::cpu::Cpu;

fn main() {
    println!("Starting CPU");

    let mut cpu = Cpu::new();
    cpu.initialize();

    let data = load_game("./chip_8_wasm/static/roms/PONG2").unwrap();
    cpu.load_game(data);

    loop {
        // Emulate one cycle
        cpu.emulate_cycle().unwrap();

        // If the draw flag is set, update the screen
        // if(cpu.drawFlag) {
        //     drawGraphics();
        // }

        // Store key press state (Press and Release)
        // cpu.setKeys();

        //break;
    }
}


fn load_game(file_name: &str) -> io::Result<Vec<u8>> {
    println!("load_game() {}", file_name);

    let file_metadata = std::fs::metadata(file_name)?;
    println!("{} is {} bytes in size", file_name, file_metadata.len());
    // TODO ensure file size is less than 4096 - 512?
    // Since most programs written for the original system begin at memory location 512 (0x200)

    let mut f = File::open(file_name)?;

    //    let mut buffer = [0; 4096];

    let mut buffer: Vec<u8> = vec![0; file_metadata.len() as usize];
    f.read_exact(&mut buffer)?;

    // for (i, item) in buffer.iter().enumerate() {
    //     println!("{} {}", i, item);
    // }

    Ok(buffer)
}
