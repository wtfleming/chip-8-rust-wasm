use wasm_bindgen::prelude::*;
use web_sys::console;
use js_sys::DataView;
use chip_8_lib::cpu::Cpu;
use chip_8_lib::disassembler;

use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

// use std::io;
// use std::io::prelude::*;
// use std::fs::File;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

// TODO find a better way to init this with default values
// ie. can Cpu::new() be a const_fn?
// https://doc.rust-lang.org/unstable-book/language-features/const-fn.html
static mut CPU: Cpu = Cpu {
    memory: [0; 4096],
    pc: 0x200,  // Program counter starts at memory index 512 (0x200 in hex)
    v: [0; 16],
    i: 0,
    stack: [0; 16],
    sp: 0,
    display: [0; 2048],
};


#[wasm_bindgen]
pub fn update_ui() {
    let window = web_sys::window().expect("no global `window` exists");
    let document: web_sys::Document  = window.document().expect("should have a document on window");

    let memory_element = document.get_element_by_id("memorylist").unwrap();
    unsafe {
        let memory_start = CPU.pc;
        let mut memory_end = CPU.pc + 50;
        if memory_end >= CPU.memory.len() as u16 {
            memory_end = CPU.memory.len() as u16;
        }

        let mut memory_vals: Vec<String> = vec![];

        for x in (memory_start..memory_end).step_by(2) {
            let code1: u16 = CPU.memory[x as usize] as u16;
            let code2: u16 = CPU.memory[(x + 1) as usize] as u16;
            let opcode: u16 = code1 << 8 | code2;

            memory_vals.push(format!("<li>0x{:X} - {}</li>", x, disassembler::disassemble(opcode)));
        }
        let output = memory_vals.join("");
        memory_element.set_inner_html(&output);
    }

    let registers_element = document.get_element_by_id("registers").unwrap();
    unsafe {
        let mut registers: Vec<String> = vec![];

        for (idx, e) in CPU.v.iter().enumerate() {
            registers.push(format!("v{}: {}", idx.to_string(), e.to_string()));
        }
        registers.push(format!("I: {}", CPU.i));

        let output = registers.join("<br />");
        registers_element.set_inner_html(&output);
    }

    let misc_element = document.get_element_by_id("misc").unwrap();
    unsafe {
        misc_element.set_inner_html(format!("PC: {} - 0x{:X} <br />", CPU.pc, CPU.pc).as_str());
    }

}


#[wasm_bindgen]
pub fn emulate_cycle() {

    // TODO - if an error return the error string and let javascript stop the emulator?
    
    // This should be getting called at about 60hz, so emulate 10 cycles, and decrement the timer by 1
    // TODO should emulate 10 cycles to get close to 500hz?

    // Maybe rename this to tick()?


    unsafe {
        //        CPU.emulate_cycle().map_err(|err| err.to_string());
        //let _foo = CPU.emulate_cycle().map_err(|err| console::error_1(&JsValue::from_str(err.message.as_str())));

        match CPU.emulate_cycle() {
            Ok(_) => (),
            Err(e) => console::error_1(&JsValue::from_str(e.message.as_str()))
        }
    }

    
    // let window = web_sys::window().expect("no global `window` exists");
    // let document: web_sys::Document  = window.document().expect("should have a document on window");

//    console::log_1(&JsValue::from_str(document.to_str()));
//    println!("{:?}", document);
//    let body = document.body().expect("document should have a body");




    // Update registers in UI
    

    // let val = document.get_element_by_id("calls").unwrap();
    // unsafe {
    // //     let string_list = vec!["Foo".to_string(),"Bar".to_string()];
    // // let joined = string_list.join("-");

    //     let code1: u16 = CPU.memory[CPU.pc as usize] as u16;
    //     let code2: u16 = CPU.memory[(CPU.pc + 1) as usize] as u16;
    //     let opcode: u16 = code1 << 8 | code2;

    //     //let code = code1 << 8 | code2;
    //     val.set_inner_html(format!("{:X} {:X}", CPU.pc, opcode).as_str());
    // }

    //val.set_inner_html("Hello from Rust!<br/ >hi");
//    body.append_child(&val).unwrap();
    //run();
}



#[wasm_bindgen]
pub fn draw_canvas(
    ctx: &CanvasRenderingContext2d,
    // width: u32,
    // height: u32,
) -> Result<(), JsValue> {
    //console::log_1(&JsValue::from_str("Running!"));

    let width = 64;
    let height = 32;

    let mut data = Vec::with_capacity((width * height) as usize);

    unsafe {
        for x in CPU.display.iter() {
            if x == &1 {
                data.push(240); // red
                data.push(246); // green
                data.push(240); // blue
                data.push(255); // alpha
            } else {
                data.push(34); // red
                data.push(35); // green
                data.push(35); // blue
                data.push(255); // alpha
        }
    }

}

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}



#[wasm_bindgen]
pub fn run() {
    console::log_1(&JsValue::from_str("Starting CPU"));


    // let mut cpu = Cpu::new();
    // cpu.initialize();

    // let data = load_game("c8games/PONG2").unwrap();
    // cpu.load_game(data);
    //    cpu.load_game("c8games/PONG2");

    //cpu.emulate_cycle();


    // loop {
    //     // Emulate one cycle
    //     cpu.emulate_cycle();

    //     // If the draw flag is set, update the screen
    //     // if(cpu.drawFlag) {
    //     //     drawGraphics();
    //     // }

    //     // Store key press state (Press and Release)
    //     // cpu.setKeys();


    //     //break;
    // }
}

#[wasm_bindgen]
pub fn load_game_js(data: DataView) {
    console::log_1(&JsValue::from_str("load_game_js()"));

    //    notify_loaded();

    let mut data_vec: Vec<u8> = Vec::with_capacity(data.byte_length());


    // TODO Is there a better way to get the DataView data into the vec?
    for idx in 0..data.byte_length() {
        //console::log_1(&JsValue::from_f64(data.get_uint8(idx) as f64));
        data_vec.push(data.get_uint8(idx));
    }

    unsafe {
        CPU.load_game(data_vec);
    }

    console::log_1(&data.buffer());

}
