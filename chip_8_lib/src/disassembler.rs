pub fn disassemble(opcode: u16) -> String {
    match opcode {
        0x00EE => {
            // 00EE - RET
            // Return from a subroutine.
            String::from("RET")
        }
        0x1000..=0x1FFF => {
            // 1nnn - JP addr
            // Jump to location nnn.
            let address = opcode & 0x0FFF;
            format!("JP 0x{:X}", address)
        }
        0x2000..=0x2FFF => {
            // 2nnn - CALL addr
            // Call subroutine at nnn.
            let address = opcode & 0x0FFF;
            format!("CALL 0x{:X}", address)
        }

        0x3000..=0x3FFF => {
            // 3xkk - SE Vx, byte
            // Skip next instruction if Vx = kk.
            let x = (opcode & 0x0F00) >> 8;
            let kk = opcode & 0x00FF;
            format!("SE V{} {} ", x, kk)
        }
        0x6000..=0x6FFF => {
            // 6xkk - LD Vx, byte
            // The interpreter puts the value kk into register Vx.
            let x = (opcode & 0x0F00) >> 8;
            let kk = opcode & 0x00FF;
            format!("LD V{} {} ", x, kk)
        }
        0x7000..=0x7FFF => {
            // 7xkk - ADD Vx, byte
            // Set Vx = Vx + kk.
            let x = (opcode & 0x0F00) >> 8;
            format!("ADD V{} {} ", x, opcode & 0x00FF)
        }

        0xA000..=0xAFFF => {
            // Annn - LD I, addr
            // The value of register I is set to nnn.
            format!("LD I {} ", opcode & 0x0FFF)
        }
        0xD000..=0xDFFF => {
            // Dxyn - DRW Vx, Vy, nibble
            // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 4;
            let nibble = opcode & 0x000F;
            format!("DRW V{} V{} {} ", x, y, nibble)
        }
        _ => format!("??? {:X}", opcode)
    }
}
