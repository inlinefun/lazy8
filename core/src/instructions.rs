use crate::CHIP8;

impl CHIP8 {
    /// clear screen
    ///
    /// set the entire 'Display' to be blank
    /// force 'Display' state to be 'false'
    pub(crate) fn clear_screen(&mut self) {
        self.display = [false; 64 * 32]
    }

    /// jump to
    ///
    /// jump to an instruction
    /// which is set 'PC' to be 'nnn'
    pub(crate) fn jump_to(&mut self, nnn: u16) {
        self.pc = nnn
    }

    /// call a subroutine
    ///
    /// push the current 'PC' to the stack
    /// then set 'PC' to 'nnn'
    pub(crate) fn call_subroutine(&mut self, nnn: u16) {
        self.stack.push(self.pc);
        self.pc = nnn;
    }

    /// return a subroutine
    ///
    /// pop the 'Stack', set the value of the popped 'Stack' element to the 'PC'
    pub(crate) fn return_subroutine(&mut self) {
        match self.stack.pop() {
            Some(value) => self.pc = value,
            None => {
                log::debug!("stack is empty, can not pop it");
                log::error!("Bad instruction: Return Subroutine");
            }
        };
    }

    /// skip an instruction
    ///
    /// which is increment 'PC' by 2
    /// if value at 'Register' 'x', is equal to 'nn'
    pub(crate) fn skip_if_equal(&mut self, x: u16, nn: u16) {
        if self.registers[x as usize] == nn as u8 {
            self.pc += 2;
        }
    }

    /// skip an instruction
    ///
    /// which is increment 'PC' by 2
    /// if value at 'Register' 'x', is not equal to 'nn'
    pub(crate) fn skip_if_not_equal(&mut self, x: u16, nn: u16) {
        if self.registers[x as usize] != nn as u8 {
            self.pc += 2;
        }
    }

    /// skip an instruction
    ///
    /// which is increment 'PC' by 2
    /// if value at 'Register' 'x', is equal to the value at 'Register' 'y'
    pub(crate) fn skip_if_register_equal(&mut self, x: u16, y: u16) {
        if self.registers[x as usize] == self.registers[y as usize] {
            self.pc += 2;
        }
    }

    /// skip an instruction
    ///
    /// which is increment 'PC' by 2
    /// if value at 'Register' 'x', is not equal to the value at 'Register' 'y'
    pub(crate) fn skip_if_register_not_equal(&mut self, x: u16, y: u16) {
        if self.registers[x as usize] != self.registers[y as usize] {
            self.pc += 2;
        }
    }

    /// set the value at 'Register' 'x' to 'nn'
    pub(crate) fn set_value(&mut self, x: u16, nn: u16) {
        self.registers[x as usize] = nn as u8;
    }

    /// add 'nn' to the value at 'Register' 'x'
    pub(crate) fn add_value(&mut self, x: u16, nn: u16) {
        self.registers[x as usize] += nn as u8
    }

    /// set index register 'i' to 'nnn'
    pub(crate) fn set_index(&mut self, nnn: u16) {
        self.i = nnn;
    }

    /// display
    pub(crate) fn display(&mut self, x: u16, y: u16, n: u16) {
        // get x and y screen coordinates
        let x_coord = (self.registers[x as usize] % 64) as u16;
        let y_coord = (self.registers[y as usize] % 32) as u16;

        // set register[F] to be 0
        self.registers[0xF as usize] = 0;

        for i in 0..n {
            let nth_byte = self.memory[(self.i + i) as usize];
            for bit in 0..8 {
                if nth_byte & (0x80 >> bit) != 0 {
                    let current_x_coord = x_coord + bit;
                    let current_y_coord = y_coord + i;
                    let index = current_x_coord + (current_y_coord * 64);
                    if self.display[index as usize] {
                        self.registers[0xF as usize] = 1;
                    }
                    self.display[index as usize] ^= true;
                }
            }
        }
    }
}
