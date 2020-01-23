
// http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
// https://en.wikipedia.org/wiki/CHIP-8
// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
// https://blog.scottlogic.com/2017/12/13/chip8-emulator-webassembly-rust.html





// CHIP-8 was most commonly implemented on 4K systems, such as the Cosmac VIP and the Telmac 1800. These machines had 4096 (0x1000) memory locations, all of which are 8 bits (a byte) which is where the term CHIP-8 originated. However, the CHIP-8 interpreter itself occupies the first 512 bytes of the memory space on these machines. For this reason, most programs written for the original system begin at memory location 512 (0x200) and do not access any of the memory below the location 512 (0x200). The uppermost 256 bytes (0xF00-0xFFF) are reserved for display refresh, and the 96 bytes below that (0xEA0-0xEFF) were reserved for the call stack, internal use, and other variables.

// In modern CHIP-8 implementations, where the interpreter is running natively outside the 4K memory space, there is no need to avoid the lower 512 bytes of memory (0x000-0x200), and it is common to store font data there.


// 35 opcodes   http://en.wikipedia.org/wiki/CHIP-8#Opcode_table
// CHIP-8 has 35 opcodes, which are all two bytes long and stored big-endian. The opcodes are listed below, in hexadecimal and with the following symbols:


// CPU registers: The Chip 8 has 15 8-bit general purpose registers named V0,V1 up to VE. The 16th register is used  for the ‘carry flag’. Eight bits is one byte so we can use an unsigned char for this purpose:



// There is an Index register I and a program counter (pc) which can have a value from 0x000 to 0xFFF

// The systems memory map:
// 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
// 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
// 0x200-0xFFF - Program ROM and work RAM

// The graphics system: The chip 8 has one instruction that draws sprite to the screen. Drawing is done in XOR mode and if a pixel is turned off as a result of drawing, the VF register is set. This is used for collision detection.

// The graphics of the Chip 8 are black and white and the screen has a total of 2048 pixels (64 x 32). This can easily be implemented using an array that hold the pixel state (1 or 0):

// Interupts and hardware registers. The Chip 8 has none, but there are two timer registers that count at 60 Hz. When set above zero they will count down to zero.

// The system’s buzzer sounds whenever the sound timer reaches zero.

// It is important to know that the Chip 8 instruction set has opcodes that allow the program to jump to a certain address or call a subroutine. While the specification don’t mention a stack, you will need to implement one as part of the interpreter yourself. The stack is used to remember the current location before a jump is performed. So anytime you perform a jump or call a subroutine, store the program counter in the stack before proceeding. The system has 16 levels of stack and in order to remember which level of the stack is used, you need to implement a stack pointer (sp).

// Finally, the Chip 8 has a HEX based keypad (0x0-0xF), you can use an array to store the current state of the key.



// LOADING FILES
// For this reason, most programs written for the original system begin at memory location 512 (0x200) and do not access any of the memory below the location 512 (0x200). The uppermost 256 bytes (0xF00-0xFFF) are reserved for display refresh, and the 96 bytes below that (0xEA0-0xEFF) were reserved for the call stack, internal use, and other variables.

// In modern CHIP-8 implementations, where the interpreter is running natively outside the 4K memory space, there is no need to avoid the lower 512 bytes of memory (0x000-0x200), and it is common to store font data there.


#[derive(Debug)]
pub struct EmulateCycleError {
    pub message: String,
}

// impl EmulateCycleError {
//     fn description(&self) -> &str {
//         &self.string
//     }
// }


pub struct Cpu {
    // Memory
    pub memory: [u8; 4096],

    // Program Counter
    pub pc: u16,

    // 16 general purpose 8-bit registers, usually referred to as Vx, where x is a hexadecimal digit (0 through F)
    pub v: [u8; 16],

    // Index register
    pub i: u16,

    // The stack is an array of 16 16-bit values
    pub stack: [u16; 16],
    // Stack pointer
    pub sp: u8,

    // 64x32 pixels
    pub display: [bool; 2048],
}


impl Default for Cpu {
    fn default() -> Self {
        Cpu { memory: [0; 4096],
              pc: 0x200,  // Program counter starts at memory index 512 (0x200 in hex)
              v: [0; 16],
              i: 0,
              stack: [0; 16],
              sp: 0,
              display: [false; 2048],
        }
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { memory: [0; 4096],
              pc: 0x200,  // Program counter starts at memory index 512 (0x200 in hex)
              v: [0; 16],
              i: 0,
              stack: [0; 16],
              sp: 0,
              display: [false; 2048],
        }
    }

    pub fn initialize(&mut self) {
        println!("initialize()");

        // Initialize registers and memory once
    }

    pub fn load_game(&mut self, data: Vec<u8>) {
        println!("load_game()");
        //println!("load_game() {:?}", data);

        for (idx, item) in data.iter().enumerate() {
            self.memory[idx + 512] = *item;
        }
    }



    //    pub fn emulate_cycle(&mut self) -> Result<(), &'static str> {
    pub fn emulate_cycle(&mut self) -> Result<(), EmulateCycleError> {
        // println!("{:X}", opcode);

        let opcode: u16 = self.fetch_current_opcode();

        match opcode {
            0x00EE => {
                // 00EE - RET
                // Return from a subroutine.
                // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
                println!("sp: {:X}", self.sp);
                println!("val: {:X}", self.stack[self.sp as usize]);

                self.pc = self.stack[self.sp as usize];
                self.pc += 2;

                self.stack[self.sp as usize] = 0xBEEF;
                self.sp -= 1;
            },


            0x1000 ..= 0x1FFF => {
                // 1nnn - JP addr
                // Jump to location nnn.
                self.pc = opcode & 0x0FFF;
            },
            0x2000 ..= 0x2FFF => {
                // 2nnn - CALL addr
                // Call subroutine at nnn.
                // Increment the stack pointer, put the current program counter on the top of the stack,
                // then the program counter is then set to nnn.
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = extract_address_from_opcode(opcode);

                // TODO better error handling if there was a stack overflow?
                println!("call subroutine at {:X}", opcode);
            },

            0x3000 ..= 0x3FFF => {
                // 3xkk - SE Vx, byte
                // Skip next instruction if Vx = kk.
                // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
                let x = (opcode & 0x0F00) >> 8;
                let kk = opcode & 0x00FF;
                if self.v[x as usize] == kk as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x6000 ..= 0x6FFF => {
                // 6xkk - LD Vx, byte
                // The interpreter puts the value kk into register Vx.
                let x = (opcode & 0x0F00) >> 8;
                let kk = opcode & 0x00FF;
                self.v[x as usize] = kk as u8;
                self.pc += 2;
            },
            0x7000 ..= 0x7FFF => {
                // 7xkk - ADD Vx, byte
                // Set Vx = Vx + kk.
                let x = (opcode & 0x0F00) >> 8;
                let kk = opcode & 0x00FF;
                self.v[x as usize] += kk as u8;
                self.pc += 2;
            },


            0xA000 ..= 0xAFFF => {
                // Annn - LD I, addr
                // Set I = nnn.
                // The value of register I is set to nnn.

                self.i = opcode & 0x0FFF;
                self.pc += 2;
            },

            // TODO implement later
            // DBC1 opcode not handled


            0xD000 ..= 0xDFFF => {
                // Dxyn
                // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
                // Each row of 8 pixels is read as bit-coded starting from memory location I; I value doesn’t change after the execution of this instruction.
                // As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that doesn’t happen


                // Dxyn - DRW Vx, Vy, nibble
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

                // The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen.
                // If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
                // If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen.
                // See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.

                let start_x = self.v[((opcode & 0x0F00) >> 8) as usize] as usize;
                let start_y = self.v[((opcode & 0x00F0) >> 4) as usize] as usize;
                let height = (opcode & 0x000F) as usize;

                let mut pixel_changed = false;
                let mut current_loc = self.i;
                // println!("height: {} start_x: {} start_y: {} ", height, start_x, start_y);
                for row in 0..height {
                    let width = 64;
                    let pixel_data :u8 = self.memory[current_loc as usize];
                    for x in (0..8).rev() {
                        let bit_value: bool = (pixel_data & (1 << (7 - x))) != 0;
                        let pixel_to_change = (width * (row + start_y) + x + start_x) as usize;

                        // println!("x: {} row: {} pixel_to_change: {} bit_value: {}", x, row, pixel_to_change, bit_value);
                        if self.display[pixel_to_change] != bit_value {
                            pixel_changed = true;
                        }

                        self.display[pixel_to_change] = bit_value;
                    }
                    current_loc += 1;
                }

                if pixel_changed {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.pc += 2;
            }

            _ => {
                //println!("{:X} opcode not handled", opcode);
                self.pc += 2;

                let error = EmulateCycleError { message: format!("{:X} opcode not handled", opcode) };
                return Err(error);
            }
        }


        // TODO Update timers - should happen at start, since an unhandled opcode won't get here

        Ok(())
    }



    fn fetch_current_opcode(&self) -> u16 {
        // Instructions are 2 bytes but memory locations are only 1 byte.
        // We need to merge the two bytes in memory to construct the opcode.

        // For example, assuming memory is represented like as an array with
        // two one byte u8's [u8; 2] and contents [0x22, 0xF6]
        // We need to get the individual bytes and merge them into a u16.

        // Get the first byte value (0x22) and convert it to a u16
        // It's u16 binary representation would be
        // 0000000000100010
        // We then shift left 8 bits like this  0x22 << 8 which results in
        // 0010001000000000

        // Next we get the second byte value (0xF6).
        // 0000000011110110

        // Then use a bitwise OR to merge them to get
        // 0010001011110110
        // Which in hexadecimal is represented at 0x22F6, the correct merge of [0x22, 0xF6]


        let code1: u16 = self.memory[self.pc as usize] as u16;
        let code2: u16 = self.memory[(self.pc + 1) as usize] as u16;
        // let opcode: u16 = code1 << 8 | code2;
        // opcode
        code1 << 8 | code2
    }

}

fn extract_address_from_opcode(opcode: u16) -> u16 {
    // We only want the last 12 bits, we can use a bitwise AND to extract them
    opcode & 0x0FFF
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extracts_an_address_from_an_opcode() {
        assert_eq!(extract_address_from_opcode(0x22F6), 0x02F6);
        assert_eq!(extract_address_from_opcode(0x22F6), 758);
    }
}
