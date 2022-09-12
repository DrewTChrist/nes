const MEMORY_SIZE: usize = 0xFFFF;
const PROGRAM_ROM: usize = 0x8000;
const STACK_SIZE: usize = 0xFF;
const STACK_BEG: usize = 0x01FF;
const STACK_END: usize = 0x0100;

/// Opcode addressing modes
#[derive(Copy, Clone)]
enum AddressMode {
    Immediate(u16),
    ZeroPage(u16),
    ZeroPageX(u16),
    ZeroPageY(u16),
    Absolute(u16),
    AbsoluteX(u16),
    AbsoluteY(u16),
    IndirectX(u16),
    IndirectY(u16),
    NoMode,
}

impl AddressMode {
    const IMMEDIATE: AddressMode = AddressMode::Immediate(1);
    const ZERO_PAGE: AddressMode = AddressMode::ZeroPage(1);
    const ZERO_PAGE_X: AddressMode = AddressMode::ZeroPageX(1);
    const ZERO_PAGE_Y: AddressMode = AddressMode::ZeroPageY(1);
    const ABSOLUTE: AddressMode = AddressMode::Absolute(2);
    const ABSOLUTE_X: AddressMode = AddressMode::AbsoluteX(2);
    const ABSOLUTE_Y: AddressMode = AddressMode::AbsoluteY(2);
    const INDIRECT_X: AddressMode = AddressMode::IndirectX(1);
    const INDIRECT_Y: AddressMode = AddressMode::IndirectY(1);

    fn get_pc_increment(&self) -> u16 {
        match self {
            AddressMode::Immediate(n) => *n,
            AddressMode::ZeroPage(n) => *n,
            AddressMode::ZeroPageX(n) => *n,
            AddressMode::ZeroPageY(n) => *n,
            AddressMode::Absolute(n) => *n,
            AddressMode::AbsoluteX(n) => *n,
            AddressMode::AbsoluteY(n) => *n,
            AddressMode::IndirectX(n) => *n,
            AddressMode::IndirectY(n) => *n,
            AddressMode::NoMode => 0,
        }
    }
}

/// Struct to hold cpu registers
#[derive(Copy, Clone)]
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
            s: STACK_BEG as u8,
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

/// Cpu Struct
pub struct Cpu {
    pub reg: Registers,
    memory: [u8; MEMORY_SIZE],
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
            memory: [0; MEMORY_SIZE],
        }
    }

    /// Returns a slice of the cpu memory
    pub fn get_mem_slice(&self, start: usize, end: usize) -> &[u8] {
        &self.memory[start..end]
    }

    /// Returns the cpu registers
    pub fn get_registers(&self) -> Registers {
        self.reg
    }

    pub fn get_stack(&self) -> &[u8] {
        self.get_mem_slice(STACK_END, STACK_BEG)
    }

    /// Loads a program into the program rom space in cpu memory
    pub fn load_program<const S: usize>(&mut self, program: [u8; S]) {
        let mut current: usize = PROGRAM_ROM as usize;
        for byte in program {
            self.memory[current] = byte;
            current += 1;
        }
    }

    /// Resets the cpu
    pub fn reset(&mut self) {
        todo!()
    }

    /// Executes a cycle on the cpu
    pub fn tick(&mut self) {
        let opcode = self.fetch_opcode();
        self.reg.pc += 1;
        self.execute(opcode);
    }

    pub fn push_stack(&mut self, value: u8) {
        self.write_mem(self.reg.s as u16, value);
        self.reg.s = self.reg.s.wrapping_sub(1);
    }

    pub fn pop_stack(&mut self) -> u8 {
        self.reg.s = self.reg.s.wrapping_add(1);
        let value = self.read_mem(self.reg.s as u16);
        value
    }

    pub fn peek_stack(&mut self, address: u8) -> Option<u8> {
        todo!()
    }

    /// Returns the value of a location in memory
    pub fn read_mem(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    /// Returns a two byte value of a location in memory
    pub fn read_mem_u16(&self, addr: u16) -> u16 {
        let mut data: u16 = self.memory[(addr + 1) as usize] as u16;
        data <<= 8;
        data |= self.memory[addr as usize] as u16;
        data
    }

    /// Writes a byte to a location in memory
    pub fn write_mem(&mut self, addr: u16, byte: u8) {
        self.memory[addr as usize] = byte;
    }

    /// Writes two bytes to a location in memory
    pub fn write_mem_u16(&mut self, addr: u16, bytes: u16) {
        self.write_mem(addr, (bytes & 0xff) as u8);
        self.write_mem(addr + 1, (bytes >> 8) as u8);
    }

    /// Grabs the value in memory pointed at by the program counter
    fn fetch_opcode(&self) -> u8 {
        self.memory[self.reg.pc as usize]
    }

    /// Executes a program instruction
    fn execute(&mut self, opcode: u8) {
        match opcode {
            0x00 => todo!(),
            0x01 => todo!(),
            0x05 => todo!(),
            0x06 => todo!(),
            0x08 => todo!(),
            0x09 => todo!(),
            0x0a => todo!(),
            0x0d => todo!(),
            0x0e => todo!(),
            0x10 => todo!(),
            0x11 => todo!(),
            0x15 => todo!(),
            0x16 => todo!(),
            0x18 => self.clc(),
            0x19 => todo!(),
            0x1d => todo!(),
            0x1e => todo!(),
            0x20 => todo!(),
            0x21 => self.and(AddressMode::INDIRECT_X),
            0x24 => todo!(),
            0x25 => self.and(AddressMode::ZERO_PAGE_X),
            0x26 => todo!(),
            0x28 => todo!(),
            0x29 => self.and(AddressMode::IMMEDIATE),
            0x2a => todo!(),
            0x2c => todo!(),
            0x2e => todo!(),
            0x2d => self.and(AddressMode::ABSOLUTE),
            0x30 => todo!(),
            0x31 => self.and(AddressMode::INDIRECT_Y),
            0x35 => self.and(AddressMode::ZERO_PAGE_X),
            0x36 => todo!(),
            0x38 => self.sec(),
            0x39 => self.and(AddressMode::ABSOLUTE_Y),
            0x3d => self.and(AddressMode::ABSOLUTE_X),
            0x3e => todo!(),
            0x40 => todo!(),
            0x41 => todo!(),
            0x45 => todo!(),
            0x46 => todo!(),
            0x48 => todo!(),
            0x49 => todo!(),
            0x4a => todo!(),
            0x4c => todo!(),
            0x4d => todo!(),
            0x4e => todo!(),
            0x50 => todo!(),
            0x51 => todo!(),
            0x55 => todo!(),
            0x56 => todo!(),
            0x58 => self.cli(),
            0x59 => todo!(),
            0x5a => todo!(),
            0x5d => todo!(),
            0x5e => todo!(),
            0x60 => todo!(),
            0x61 => todo!(),
            0x65 => todo!(),
            0x66 => todo!(),
            0x68 => todo!(),
            0x69 => todo!(),
            0x6a => todo!(),
            0x6c => todo!(),
            0x6d => todo!(),
            0x6e => todo!(),
            0x70 => todo!(),
            0x71 => todo!(),
            0x75 => todo!(),
            0x78 => self.sei(),
            0x79 => todo!(),
            0x7d => todo!(),
            0x7e => todo!(),
            0x81 => todo!(),
            0x84 => todo!(),
            0x85 => todo!(),
            0x86 => todo!(),
            0x88 => self.dey(),
            0x8a => todo!(),
            0x8c => todo!(),
            0x8d => todo!(),
            0x8e => todo!(),
            0x90 => todo!(),
            0x91 => todo!(),
            0x94 => todo!(),
            0x95 => todo!(),
            0x96 => todo!(),
            0x98 => todo!(),
            0x99 => todo!(),
            0x9a => todo!(),
            0x9d => todo!(),
            0xa0 => self.ldy(AddressMode::IMMEDIATE),
            0xa1 => self.lda(AddressMode::INDIRECT_X),
            0xa2 => self.ldx(AddressMode::IMMEDIATE),
            0xa4 => self.ldy(AddressMode::ZERO_PAGE),
            0xa5 => self.lda(AddressMode::ZERO_PAGE),
            0xa6 => self.ldx(AddressMode::ZERO_PAGE),
            0xa8 => self.tay(),
            0xa9 => self.lda(AddressMode::IMMEDIATE),
            0xaa => self.tax(),
            0xac => self.ldy(AddressMode::ABSOLUTE),
            0xad => self.lda(AddressMode::ABSOLUTE),
            0xae => self.ldx(AddressMode::ABSOLUTE),
            0xb0 => todo!(),
            0xb1 => self.lda(AddressMode::INDIRECT_Y),
            0xb4 => self.ldy(AddressMode::ZERO_PAGE_X),
            0xb5 => self.lda(AddressMode::ZERO_PAGE_X),
            0xb6 => self.ldx(AddressMode::ZERO_PAGE_X),
            0xb8 => self.clv(),
            0xb9 => self.lda(AddressMode::ABSOLUTE_Y),
            0xba => self.tsx(),
            0xbc => self.ldy(AddressMode::ABSOLUTE_X),
            0xbd => self.lda(AddressMode::ABSOLUTE_X),
            0xbe => self.ldx(AddressMode::ABSOLUTE_X),
            0xc0 => todo!(),
            0xc1 => todo!(),
            0xc4 => todo!(),
            0xc5 => todo!(),
            0xc6 => self.dec(AddressMode::ZERO_PAGE),
            0xc8 => self.iny(),
            0xc9 => todo!(),
            0xca => self.dex(),
            0xcc => todo!(),
            0xcd => todo!(),
            0xce => self.dec(AddressMode::ABSOLUTE),
            0xd0 => todo!(),
            0xd1 => todo!(),
            0xd5 => todo!(),
            0xd6 => self.dec(AddressMode::ZERO_PAGE_X),
            0xd9 => todo!(),
            0xdd => todo!(),
            0xde => self.dec(AddressMode::ABSOLUTE_X),
            0xd8 => self.cld(),
            0xe0 => todo!(),
            0xe1 => todo!(),
            0xe4 => todo!(),
            0xe5 => todo!(),
            0xe6 => self.inc(AddressMode::ZERO_PAGE),
            0xe8 => self.inx(),
            0xe9 => todo!(),
            0xea => self.nop(),
            0xec => todo!(),
            0xed => todo!(),
            0xee => self.inc(AddressMode::ABSOLUTE),
            0xf0 => todo!(),
            0xf1 => todo!(),
            0xf5 => todo!(),
            0xf6 => self.inc(AddressMode::ZERO_PAGE_X),
            0xf8 => self.sed(),
            0xf9 => todo!(),
            0xfd => todo!(),
            0xfe => self.inc(AddressMode::ABSOLUTE_X),
            _ => {}
        }
    }

    /// Gets an address for an instruction based on an addressing mode
    fn get_address(&mut self, address_mode: AddressMode) -> u16 {
        let mut addr: u16 = 0;
        match address_mode {
            AddressMode::Immediate(u16) => {
                addr = self.reg.pc;
            }
            AddressMode::ZeroPage(u16) => {
                addr = self.read_mem(self.reg.pc) as u16;
            }
            AddressMode::ZeroPageX(u16) => {
                addr = self.read_mem(self.reg.pc).wrapping_add(self.reg.x) as u16;
            }
            AddressMode::ZeroPageY(u16) => {
                addr = self.read_mem(self.reg.pc).wrapping_add(self.reg.y) as u16;
            }
            AddressMode::Absolute(u16) => {
                addr = self.read_mem_u16(self.reg.pc);
            }
            AddressMode::AbsoluteX(u16) => {
                addr = self
                    .read_mem_u16(self.reg.pc)
                    .wrapping_add(self.reg.x as u16);
            }
            AddressMode::AbsoluteY(u16) => {
                addr = self
                    .read_mem_u16(self.reg.pc)
                    .wrapping_add(self.reg.y as u16);
            }
            AddressMode::IndirectX(u16) => {
                let base = self.read_mem(self.reg.pc);
                let ptr: u8 = (base as u8).wrapping_add(self.reg.x);
                let lo = self.read_mem(ptr as u16);
                let hi = self.read_mem(ptr.wrapping_add(1) as u16);
                addr = (hi as u16) << 8 | (lo as u16)
            }
            AddressMode::IndirectY(u16) => {
                let base = self.read_mem(self.reg.pc);
                let lo = self.read_mem(base as u16);
                let hi = self.read_mem((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                addr = deref_base.wrapping_add(self.reg.y as u16);
            }
            AddressMode::NoMode => {}
        }
        addr
    }

    fn update_flag(&mut self, flag: u8, condition: bool) {
        let flag_bit: u8 = 1 << flag;
        if condition {
            self.reg.p |= flag_bit;
        } else {
            self.reg.p &= !flag_bit;
        }
    }

    fn and(&mut self, address_mode: AddressMode) {
        let address = self.get_address(address_mode);
        self.reg.a &= self.read_mem(address);
        //self.reg.pc += 1;
        self.reg.pc += address_mode.get_pc_increment();
        self.update_flag(1, self.reg.a == 0);
        self.update_flag(7, self.reg.a & 0b1000_0000 != 0);
    }

    fn clc(&mut self) {
        self.update_flag(0, false);
    }

    fn cld(&mut self) {
        self.update_flag(3, false);
    }

    fn cli(&mut self) {
        self.update_flag(2, false);
    }

    fn clv(&mut self) {
        self.update_flag(6, false);
    }

    fn dec(&mut self, address_mode: AddressMode) {
        let address = self.get_address(address_mode);
        let value = self.read_mem(address).wrapping_sub(1);
        //self.reg.pc += 1;
        self.reg.pc += address_mode.get_pc_increment();
        self.write_mem(address, value);
        self.update_flag(1, value == 0);
        self.update_flag(7, value & 0b1000_0000 != 0);
    }

    fn dex(&mut self) {
        self.reg.x = self.reg.x.wrapping_sub(1);
        self.update_flag(1, self.reg.x == 0);
        self.update_flag(7, self.reg.x & 0b1000_0000 != 0);
    }

    fn dey(&mut self) {
        self.reg.y = self.reg.y.wrapping_sub(1);
        self.update_flag(1, self.reg.y == 0);
        self.update_flag(7, self.reg.y & 0b1000_0000 != 0);
    }

    fn inc(&mut self, address_mode: AddressMode) {
        let address = self.get_address(address_mode);
        let value = self.read_mem(address).wrapping_add(1);
        //self.reg.pc += 1;
        self.reg.pc += address_mode.get_pc_increment();
        self.write_mem(address, value);
        self.update_flag(1, value == 0);
        self.update_flag(7, value & 0b1000_0000 != 0);
    }

    fn inx(&mut self) {
        self.reg.x = self.reg.x.wrapping_add(1);
        self.update_flag(1, self.reg.x == 0);
        self.update_flag(7, self.reg.x & 0b1000_0000 != 0);
    }

    fn iny(&mut self) {
        self.reg.y = self.reg.y.wrapping_add(1);
        self.update_flag(1, self.reg.y == 0);
        self.update_flag(7, self.reg.y & 0b1000_0000 != 0);
    }

    fn lda(&mut self, address_mode: AddressMode) {
        let address = self.get_address(address_mode);
        self.reg.a = self.read_mem(address);
        //self.reg.pc += 1;
        self.reg.pc += address_mode.get_pc_increment();
        self.update_flag(1, self.reg.a == 0);
        self.update_flag(7, self.reg.a & 0b1000_0000 != 0);
    }

    fn ldx(&mut self, address_mode: AddressMode) {
        let address = self.get_address(address_mode);
        self.reg.x = self.read_mem(address);
        //self.reg.pc += 1;
        self.reg.pc += address_mode.get_pc_increment();
        self.update_flag(1, self.reg.x == 0);
        self.update_flag(7, self.reg.x & 0b1000_0000 != 0);
    }

    fn ldy(&mut self, address_mode: AddressMode) {
        let address = self.get_address(address_mode);
        self.reg.y = self.read_mem(address);
        //self.reg.pc += 1;
        self.reg.pc += address_mode.get_pc_increment();
        self.update_flag(1, self.reg.y == 0);
        self.update_flag(7, self.reg.y & 0b1000_0000 != 0);
    }

    fn nop(&self) {}

    fn sec(&mut self) {
        self.update_flag(0, true);
    }

    fn sed(&mut self) {
        self.update_flag(3, true);
    }

    fn sei(&mut self) {
        self.update_flag(2, true);
    }

    fn sta(&mut self, address_mode: AddressMode) {}

    fn stx(&mut self, address_mode: AddressMode) {}

    fn sty(&mut self, address_mode: AddressMode) {}

    fn tax(&mut self) {
        self.reg.x = self.reg.a;
        self.update_flag(1, self.reg.x == 0);
        self.update_flag(7, self.reg.x & 0b1000_0000 != 0);
    }

    fn tay(&mut self) {
        self.reg.y = self.reg.a;
        self.update_flag(1, self.reg.y == 0);
        self.update_flag(7, self.reg.y & 0b1000_0000 != 0);
    }

    fn tsx(&mut self) {
        self.reg.x = self.reg.s;
        self.update_flag(1, self.reg.x == 0);
        self.update_flag(7, self.reg.x & 0b1000_0000 != 0);
    }

    fn txa(&mut self) {}

    fn txs(&mut self) {}

    fn tya(&mut self) {}
}
