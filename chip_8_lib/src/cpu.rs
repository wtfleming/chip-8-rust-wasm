use crate::emulate_cycle_error::EmulateCycleError;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;


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
    pub display: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],

    // Delay timer
    pub dt: u8,

    // Sound timer
    pub st: u8,

    // Keyboard
    pub keys: [bool; 16],
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu::new()
    }
}

pub const CHIP8_FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];


impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            memory: [0; 4096],
            pc: 0x200,  // Program counter starts at memory index 512 (0x200 in hex)
            v: [0; 16],
            i: 0,
            stack: [0; 16],
            sp: 0,
            display: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            dt: 0,
            st: 0,
            keys: [false; 16]
        };
// // 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
//        cpu.memory[..80].clone_from_slice(&CHIP8_FONT_SET[..80]);

        cpu.memory[0x050..0x0A0].clone_from_slice(&CHIP8_FONT_SET[..80]);
        cpu
    }

    pub fn initialize(&mut self) {
        for (i, item) in CHIP8_FONT_SET.iter().enumerate() {
            self.memory[i] = *item;
        }
    }

    pub fn load_game(&mut self, data: Vec<u8>) {
        println!("load_game()");
        //println!("load_game() {:?}", data);

        for (idx, item) in data.iter().enumerate() {
            self.memory[idx + 512] = *item;
        }
    }

    pub fn emulate_cycle(&mut self) -> Result<(), EmulateCycleError> {
        let opcode: u16 = self.fetch_current_opcode();
        match opcode {
            0x00E0 => {
                // 00E0 - CLS
                // Clear the display.
                for pixel in self.display.iter_mut() {
                    *pixel = 0;
                }
                self.pc += 2;
            }
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
                self.pc = opcode & 0x0FFF;

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
            0x4000..=0x4FFF => {
                // 4xkk - SNE Vx, byte
                // Skip next instruction if Vx != kk.
                //The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
                let x = (opcode & 0x0F00) >> 8;
                let kk = opcode & 0x00FF;
                if self.v[x as usize] != kk as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x5000..=0x5FFF => {
                // 5xy0 - SE Vx, Vy
                // Skip next instruction if Vx = Vy.
                let x = (opcode & 0x0F00) >> 8;
                let y = (opcode & 0x00F0) >> 4;
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
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
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let kk = (opcode & 0x00FF) as u8;

                let (result, _) = self.v[x].overflowing_add(kk);
                self.v[x] = result;
                self.pc += 2;
            },
            0x8000..=0x8FFF => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                let subcode = opcode & 0x000F;
                match subcode {
                    0 => {
                        // 8xy0 - LD Vx, Vy
                        // Set Vx = Vy.
                        self.v[x] = self.v[y];
                        self.pc += 2;
                    }
                    1 => {
                        // 8xy1 - OR Vx, Vy
                        // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
                        self.v[x] |= self.v[y];
                        self.pc += 2;
                    }
                    2 => {
                        // 8xy2 - AND Vx, Vy
                        // Set Vx = Vx AND Vy.
                        // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
                        self.v[x] &= self.v[y];
                        self.pc += 2;
                    }
                    3 => {
                        // 8xy3 - XOR Vx, Vy
                        // Set Vx = Vx XOR Vy.
                        // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
                        self.v[x] ^= self.v[y];
                        self.pc += 2;
                    }
                    4 => {
                        // 8xy4 - ADD Vx, Vy
                        // Set Vx = Vx + Vy, set VF = carry.
                        // The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
                        let (value, did_overflow) = self.v[x].overflowing_add(self.v[y]);
                        if did_overflow {
                            self.v[0xF] = 1;
                        }
                        self.v[x] = value;
                        self.pc += 2;
                    }
                    5 => {
                        // 8xy5 - SUB Vx, Vy
                        // Set Vx = Vx - Vy, set VF = NOT borrow.
                        // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
                        let (value, _did_overflow) = self.v[x].overflowing_sub(self.v[y]);
                        if self.v[x] > self.v[y] {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }
                        self.v[x] = value;
                        self.pc += 2;
                    }
                    6 => {
                        // 8xy6 - SHR Vx {, Vy}
                        // Set Vx = Vx SHR 1.
                        // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
                        self.v[0xF] = self.v[x] & 0x1;
                        self.v[x] >>= 1;
                        self.pc += 2;
                    }
                    7 => {
                        // 8xy7 - SUBN Vx, Vy
                        // Set Vx = Vy - Vx, set VF = NOT borrow.
                        // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
                        let (value, did_overflow) = self.v[x].overflowing_sub(self.v[y]);
                        if did_overflow {
                            self.v[0xF] = 0;
                        } else {
                            self.v[0xF] = 1;
                        }
                        self.v[x] = value;
                        self.pc += 2;
                    }
                    0xE => {
                        // 8xyE - SHL Vx {, Vy}
                        // Set Vx = Vx SHL 1.
                        // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
                        self.v[0xF] = self.v[x] & 0x80;
                        self.v[x] <<= 1;
                        self.pc += 2;
                    }
                    _ => {
                        self.pc += 2;
                        let error = EmulateCycleError { message: format!("{:X} opcode not handled", opcode) };
                        return Err(error);
                    }
                }
            }
            0x9000..=0x9FFF => {
                // 9xy0 - SNE Vx, Vy
                // Skip next instruction if Vx != Vy.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.v[x] != self.v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0xA000..=0xAFFF => {
                // Annn - LD I, addr
                // Set I = nnn.
                // The value of register I is set to nnn.

                self.i = opcode & 0x0FFF;
                self.pc += 2;
            },
            0xB000..=0xBFFF => {
                // Bnnn - JP V0, addr
                // Jump to location nnn + V0.
                // The program counter is set to nnn plus the value of V0.
                let address = opcode & 0x0FFF;
                self.pc = (self.v[0x0] as u16) + address;
            }
            0xC000..=0xCFFF => {
                // Cxkk - RND Vx, byte
                // Set Vx = random byte AND kk.
                // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx.
                let x = (opcode & 0x0F00) >> 8;
                let kk = (opcode & 0x00FF) as u8;

                let mut buf = [0u8; 1];
                getrandom::getrandom(&mut buf).unwrap();
                let random = buf[0];

                self.v[x as usize] = random & kk;

                self.pc += 2;
            }
            0xD000 ..= 0xDFFF => {
                // Dxyn - DRW Vx, Vy, nibble
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                let start_x: usize = self.v[((opcode & 0x0F00) >> 8) as usize] as usize;
                let start_y: usize = self.v[((opcode & 0x00F0) >> 4) as usize] as usize;
                let height: usize = (opcode & 0x000F) as usize;

                self.v[0xF] = 0;

                let mut current_loc = self.i;
                for row in 0..height {
                    let pixel_data :u8 = self.memory[current_loc as usize];
                    for x in (0..8).rev() {
                        let new_value: u8 = pixel_data & (1 << (7 - x));
                        let new_value = new_value >> (7 - x);

                        if new_value == 1 {
                            let xi = (x + start_x) % SCREEN_WIDTH;
                            let yi = (row + start_y) % SCREEN_HEIGHT;
                            let pixel_to_change = (xi + yi * SCREEN_WIDTH) as usize;

                            let old_value: bool = self.display[pixel_to_change] == 1;
                            if old_value {
                                self.v[0xF] = 1;
                            }

                            self.display[pixel_to_change] = ((new_value == 1) ^ old_value) as u8;
                        }
                    }
                    current_loc += 1;
                }
                self.pc += 2;
            }
            0xE000 ..= 0xEFFF => {
                let x = (opcode & 0x0F00) >> 8;
                let code = opcode & 0x00FF;
                match code {
                    0x9E => {
                        // Ex9E - SKP Vx
                        // Skip next instruction if key with the value of Vx is pressed.
                        if self.keys[self.v[x as usize] as usize]{
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    0xA1 => {
                        // ExA1 - SKNP Vx
                        // Skip next instruction if key with the value of Vx is not pressed.
                        // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position;
                        if !self.keys[self.v[x as usize] as usize]{
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    _ => {
                        self.pc += 2;
                        let error = EmulateCycleError { message: format!("{:X} opcode not handled", opcode) };
                        return Err(error);
                    }
                }
            }
            0xF000 ..= 0xFFFF => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let code = opcode & 0x00FF;
                match code {
                    0x07 => {
                        // Fx07 - LD Vx, DT
                        // Set Vx = delay timer value.
                        self.v[x] = self.dt;
                    }
                    0x0A => {
                        // Fx0A - LD Vx, K
                        // Wait for a key press, store the value of the key in Vx.
                        // All execution stops until a key is pressed, then the value of that key is stored in Vx.
                        for (i, key) in self.keys.iter().enumerate() {
                            if *key {
                                self.v[x] = i as u8;
                                self.pc +=2;
                            }
                        }
                    }
                    0x15 => {
                        // Fx15 - LD DT, Vx
                        // Set delay timer = Vx.
                        self.dt = self.v[x];
                    }
                    0x18 => {
                        // Fx18 - LD ST, Vx
                        // Set sound timer = Vx.
                        self.st = self.v[x];
                    }
                    0x1E => {
                        // Fx1E - ADD I, Vx
                        // Set I = I + Vx.
                        self.i += self.v[x] as u16;
                    }
                    0x29 => {
                        // Fx29 - LD F, Vx
                        // Set I = location of sprite for digit Vx.
                        self.i = self.v[x] as u16;
                    }
                    0x33 => {
                        // Fx33 - LD B, Vx
                        // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                        self.memory[self.i as usize] = (self.v[x] / 100) as u8;
                        self.memory[(self.i + 1) as usize] = (self.v[x] / 10) as u8 % 10;
                        self.memory[(self.i + 2) as usize] = (self.v[x] % 100) as u8 % 10;
                    }
                    0x55 => {
                        // Fx55 - LD [I], Vx
                        // Store registers V0 through Vx in memory starting at location I.
                        // The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
                        for offset in 0..=x {
                            self.memory[(self.i + offset as u16) as usize] = self.v[offset];
                        }
                    }
                    0x65 => {
                        // Fx65 - LD Vx, [I]
                        // The interpreter reads values from memory starting at location I into registers V0 through Vx.
                        for offset in 0..=x {
                            self.v[offset] = self.memory[(self.i + offset as u16) as usize];
                        }
                    }
                    _ => {
                        self.pc += 2;
                        let error = EmulateCycleError { message: format!("{:X} opcode not handled", opcode) };
                        return Err(error);
                    }
                }
                self.pc += 2;
            }
            _ => {
                self.pc += 2;
                let error = EmulateCycleError { message: format!("{:X} opcode not handled", opcode) };
                return Err(error);
            }
        }

        // TODO increate the program counter by 2 here instead of in every match block?   
        //    leave it in the _ error handler match though?

        // Decrease timers
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }

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
        code1 << 8 | code2
    }

}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn addition_overflows() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 255;
        cpu.memory[0x200] = 0x70 as u8;
        cpu.memory[0x201] = 0x02;

        cpu.emulate_cycle().unwrap();
        assert_eq!(cpu.v[0], 1);
    }


//     #[test]
//     fn it_draws_a_sprite() {
//         let mut cpu = Cpu::new();
//         cpu.v[11] = 32;
//         cpu.v[12] = 0;
//         cpu.i = 746;

//         // DRW V11 V12 1
//         cpu.memory[0x2FC] = 0xDB as u8;
//         cpu.memory[0x2FD] = 0xC1 as u8;

//         cpu.pc = 0x2FC;

//         assert_eq!(cpu.display[32], 0);
//         assert_eq!(cpu.pc, 764);


//         // let s = String::from_utf8(cpu.display.to_vec()).expect("Found invalid UTF-8");
//         // println!("aaaa");
//         // println!("{}", s);

// //        println!("{:?}", cpu.display.to_vec());


//         cpu.emulate_cycle().unwrap();
// //        println!("{:?}", cpu.display.to_vec());

//         assert_eq!(cpu.display[31], 0);
//         assert_eq!(cpu.display[32], 1);
//         assert_eq!(cpu.display[32], 0);
//         assert_eq!(cpu.pc, 766);
// //        cpu.emulate_cycle().unwrap();
// //                        println!("{}", cpu.display[31]);
//     }
}
