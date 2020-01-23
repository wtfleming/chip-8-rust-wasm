
use std::io;
use std::io::prelude::*;
use std::fs::File;

use chip_8_lib::cpu::Cpu;

fn main() {
    println!("Starting CPU");

    let mut cpu = Cpu::new();
    cpu.initialize();

    // let data = load_game("../c8games/PONG2").unwrap();
    let data = load_game("./c8games/PONG2").unwrap();
    cpu.load_game(data);
    //    cpu.load_game("c8games/PONG2");

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
    f.read(&mut buffer)?;


    // for (i, item) in buffer.iter().enumerate() {
    //     println!("{} {}", i, item);
    // }

    Ok(buffer)
}


// fn load_game(file_name: &str) -> io::Result<()> {
//     println!("load_game() {}", file_name);

//     let file_metadata = std::fs::metadata(file_name)?;
//     println!("{} is {} bytes in size", file_name, file_metadata.len());
//     // TODO ensure file size is less than 4096 - 512?
//     // Since most programs written for the original system begin at memory location 512 (0x200)

//     let mut f = File::open(file_name)?;

//     //    let mut buffer = [0; 4096];

//     let mut buffer = vec![0; file_metadata.len() as usize];
//     f.read(&mut buffer)?;


//     for (i, item) in buffer.iter().enumerate() {
//         println!("{} {}", i, item);
//     }
//     Ok(())
// }

// 34  = 0x22
// 246 = 0xF6
// 107 = 0x6B
// 12  = 0xC
