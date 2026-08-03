mod instructions;

pub struct CHIP8 {
    display: [bool; 64 * 32],
    _memory: [u8; 4 * 1024],
    pc: u16,
    i: u16,
    stack: Vec<u16>,
    _delay_timer: u8,
    _sound_timer: u8,
    registers: [u8; 16],
}

struct DecodedData {
    // complete opcode, as is
    opcode: u16,
    // instruction differentiator
    instruction: u16,
    // 4 bit
    x: u16,
    // 4 bit
    y: u16,
    // 4 bit
    n: u16,
    // 8 bit
    nn: u16,
    // 12 bit
    nnn: u16,
}

impl Default for CHIP8 {
    fn default() -> Self {
        Self {
            display: [false; 64 * 32],
            _memory: [0; 4 * 1024],
            pc: 0x200, // 512 in decimal
            i: 0,
            stack: vec![],
            _delay_timer: 0,
            _sound_timer: 0,
            registers: [0; 16],
        }
    }
}

impl CHIP8 {
    pub fn load(&mut self) {}
    pub fn step(&mut self) {
        let opcode = self.fetch();
        let data = self.decode(opcode);
        self.execute(data);
    }

    fn fetch(&mut self) -> u16 {
        let first_byte = self._memory[self.pc as usize] as u16;
        let second_byte = self._memory[(self.pc + 1) as usize] as u16;
        let opcode = (first_byte << 8) | second_byte;
        self.pc += 2;
        return opcode;
    }
    fn decode(&mut self, opcode: u16) -> DecodedData {
        let instruction = (opcode & 0xF000) >> 12;
        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4;
        let n = opcode & 0x000F;
        let nn = opcode & 0x00FF;
        let nnn = opcode & 0x0FFF;

        DecodedData {
            opcode,
            instruction,
            x,
            y,
            n,
            nn,
            nnn,
        }
    }
    fn execute(&mut self, data: DecodedData) {
        match (data.instruction, data.x, data.y, data.n) {
            // clear screen
            (0x0, 0x0, 0xE, 0x0) => self.clear_screen(),

            // jump
            (0x1, _, _, _) => self.jump_to(data.nnn),

            // subroutines
            (0x2, _, _, _) => self.call_subroutine(data.nnn),
            (0x0, 0x0, 0xE, 0xE) => self.return_subroutine(),

            // skip conditionally
            (0x3, _, _, _) => self.skip_if_equal(data.x, data.nn),
            (0x4, _, _, _) => self.skip_if_not_equal(data.x, data.nn),
            (0x5, _, _, 0x0) => self.skip_if_register_equal(data.x, data.y),
            (0x9, _, _, 0x0) => self.skip_if_register_not_equal(data.x, data.y),

            // set
            (0x6, _, _, _) => self.set_value(data.x, data.nn),

            // add
            (0x7, _, _, _) => self.add_value(data.x, data.nn),

            // set index
            (0xA, _, _, _) => self.set_index(data.nnn),

            // display
            (0xD, _, _, _) => (),

            _ => {
                log::debug!("Opcode {} possibly not implemented", data.opcode);
                log::trace!("{}, {}, {}, {}", data.instruction, data.x, data.y, data.n);
                panic!("Instruction not implemented (?)")
            }
        }
    }
}
