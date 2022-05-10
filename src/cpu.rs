const MEMORY: usize = 0xFFFF;
const PROGRAM_ROM: usize = 0x8000;

pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub s: u8,
    pub p: u8,
}

impl Registers {
    fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: PROGRAM_ROM as u16,
            s: 0,
            p: 0,
        }
    }

    fn reset(&mut self, pc: u16) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.pc = pc;
        self.s = 0;
        self.p = 0;
    }
}

pub struct Cpu {
    pub reg: Registers,
    memory: [u8; MEMORY],
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            reg: Registers::new(),
            memory: [0; MEMORY],
        }
    }

    pub fn get_mem_slice(&self, start: usize, end: usize) -> &[u8] {
        &self.memory[start..end]
    }

    pub fn load_program<const S: usize>(&mut self, program: [u8; S]) {
        let mut current: usize = PROGRAM_ROM as usize;
        for byte in program {
            self.memory[current] = byte;
            current += 1;
        }
    }

    pub fn reset(&mut self) {}

    pub fn tick(&mut self) {
        let opcode = self.fetch_opcode();
        self.reg.pc += 1;
        self.execute(opcode);
    }

    fn fetch_opcode(&self) -> u8 {
        self.memory[self.reg.pc as usize]
    }

    fn execute(&mut self, opcode: u8) {
        match opcode {
            0x00 => {},
            0xa9 => {
                self.lda();
            },
            0xaa => {
                self.tax();
            },
            0xe8 => {
                self.inx();
            },
            _ => {}
        }
    }
    
    fn lda(&mut self) {
        self.reg.a = self.memory[self.reg.pc as usize];
        self.reg.pc += 1;

        if self.reg.a == 0 {
            self.reg.s |= 0b0000_0010;
        } else {
            self.reg.s &= 0b1111_1101;
        }

        if self.reg.a & 0b1000_0000 != 0 {
            self.reg.s |= 0b1000_0000;
        } else {
            self.reg.s &= 0b0111_1111;
        }
    }

    fn tax(&mut self) {
        self.reg.x = self.reg.a;

        if self.reg.x == 0 {
            self.reg.s |= 0b0000_0010;
        } else {
            self.reg.s &= 0b0111_1111;
        }

        if self.reg.x & 0b1000_0000 != 0 {
            self.reg.s |= 0b1000_0000;
        } else {
            self.reg.s &= 0b0111_1111;
        }
    }
    
    fn inx(&mut self) {
        self.reg.x = self.reg.x.overflowing_add(1).0;

        if self.reg.x == 0 {
            self.reg.s |= 0b0000_0010;
        } else {
            self.reg.s &= 0b0111_1111;
        }

        if self.reg.x & 0b1000_0000 != 0 {
            self.reg.s |= 0b1000_0000;
        } else {
            self.reg.s &= 0b0111_1111;
        }

    }
}
