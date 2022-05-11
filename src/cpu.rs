const MEMORY: usize = 0xFFFF;
const PROGRAM_ROM: usize = 0x8000;

/// Opcode addressing modes
enum AddressMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    NoMode,
}

struct OpCode {
    opcode: u16,
    instruction_name: &'static str,
    bytes: u8,
    cycles: u8,
    address_mode: AddressMode,
}

impl OpCode {
    const fn new(
        opcode: u16,
        instruction_name: &'static str,
        bytes: u8,
        cycles: u8,
        address_mode: AddressMode,
    ) -> Self {
        Self {
            opcode,
            instruction_name,
            bytes,
            cycles,
            address_mode,
        }
    }
}

const OPCODES: [OpCode; 1] = [OpCode::new(0xa9, "LDA", 2, 6, AddressMode::Immediate)];

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

/// Cpu Struct
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

    /// Returns a slice of the cpu memory
    pub fn get_mem_slice(&self, start: usize, end: usize) -> &[u8] {
        &self.memory[start..end]
    }

    /// Returns the cpu registers
    pub fn get_registers(&self) -> Registers {
        self.reg
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

    /// Grabs the value in memory pointed at by te program counter
    fn fetch_opcode(&self) -> u8 {
        self.memory[self.reg.pc as usize]
    }

    /// Executes a program instruction
    fn execute(&mut self, opcode: u8) {
        match opcode {
            0x00 => {}
            0x01 => {}
            0x05 => {}
            0x06 => {}
            0x08 => {}
            0x09 => {}
            0x0a => {}
            0x0d => {}
            0x0e => {}
            0x10 => {}
            0x11 => {}
            0x15 => {}
            0x16 => {}
            0x18 => {
                self.clc();
            }
            0x19 => {}
            0x1d => {}
            0x1e => {}
            0x20 => {}
            0x21 => {
                self.and(AddressMode::IndirectX);
            }
            0x24 => {}
            0x25 => {
                self.and(AddressMode::ZeroPage);
            }
            0x26 => {}
            0x28 => {}
            0x29 => {
                self.and(AddressMode::Immediate);
            }
            0x2a => {}
            0x2c => {}
            0x2e => {}
            0x2d => {
                self.and(AddressMode::Absolute);
            }
            0x30 => {}
            0x31 => {
                self.and(AddressMode::IndirectY);
            }
            0x35 => {
                self.and(AddressMode::ZeroPageX);
            }
            0x36 => {}
            0x38 => {
                self.sec();
            }
            0x39 => {
                self.and(AddressMode::AbsoluteY);
            }
            0x3d => {
                self.and(AddressMode::AbsoluteX);
            }
            0x3e => {}
            0x40 => {}
            0x41 => {}
            0x45 => {}
            0x46 => {}
            0x48 => {}
            0x49 => {}
            0x4a => {}
            0x4c => {}
            0x4d => {}
            0x4e => {}
            0x50 => {}
            0x51 => {}
            0x55 => {}
            0x56 => {}
            0x58 => {
                self.cli();
            }
            0x59 => {}
            0x5a => {}
            0x5d => {}
            0x5e => {}
            0x60 => {}
            0x61 => {}
            0x65 => {}
            0x66 => {}
            0x68 => {}
            0x69 => {}
            0x6a => {}
            0x6c => {}
            0x6d => {}
            0x6e => {}
            0x70 => {}
            0x71 => {}
            0x75 => {}
            0x78 => {
                self.sei();
            }
            0x79 => {}
            0x7d => {}
            0x7e => {}
            0x81 => {}
            0x84 => {}
            0x85 => {}
            0x86 => {}
            0x88 => {}
            0x8a => {}
            0x8c => {}
            0x8d => {}
            0x8e => {}
            0x90 => {}
            0x91 => {}
            0x94 => {}
            0x95 => {}
            0x96 => {}
            0x98 => {}
            0x99 => {}
            0x9a => {}
            0x9d => {}
            0xa0 => {
                self.ldy(AddressMode::Immediate);
            }
            0xa1 => {
                self.lda(AddressMode::IndirectX);
            }
            0xa2 => {
                self.ldx(AddressMode::Immediate);
            }
            0xa4 => {
                self.ldy(AddressMode::ZeroPage);
            }
            0xa5 => {
                self.lda(AddressMode::ZeroPage);
            }
            0xa6 => {
                self.ldx(AddressMode::ZeroPage);
            }
            0xa8 => {}
            0xa9 => {
                self.lda(AddressMode::Immediate);
            }
            0xaa => {
                self.tax();
            }
            0xac => {
                self.ldy(AddressMode::Absolute);
            }
            0xad => {
                self.lda(AddressMode::Absolute);
            }
            0xae => {
                self.ldx(AddressMode::Absolute);
            }
            0xb0 => {}
            0xb1 => {
                self.lda(AddressMode::IndirectY);
            }
            0xb4 => {
                self.ldy(AddressMode::ZeroPageX)
            }
            0xb5 => {
                self.lda(AddressMode::ZeroPageX);
            }
            0xb6 => {
                self.ldx(AddressMode::ZeroPageX);
            }
            0xb8 => {
                self.clv();
            }
            0xb9 => {
                self.lda(AddressMode::AbsoluteY);
            }
            0xba => {}
            0xbc => {
                self.ldy(AddressMode::AbsoluteX);
            }
            0xbd => {
                self.lda(AddressMode::AbsoluteX);
            }
            0xbe => {
                self.ldx(AddressMode::AbsoluteX);
            }
            0xc0 => {}
            0xc1 => {}
            0xc4 => {}
            0xc5 => {}
            0xc6 => {}
            0xc8 => {
                self.iny();
            }
            0xc9 => {}
            0xca => {}
            0xcc => {}
            0xcd => {}
            0xce => {}
            0xd0 => {}
            0xd1 => {}
            0xd5 => {}
            0xd6 => {}
            0xd9 => {}
            0xdd => {}
            0xde => {}
            0xd8 => {
                self.cld();
            }
            0xe0 => {}
            0xe1 => {}
            0xe4 => {}
            0xe5 => {}
            0xe6 => {
                self.inc(AddressMode::ZeroPage);
            }
            0xe8 => {
                self.inx();
            }
            0xe9 => {}
            0xea => {
                self.nop();
            }
            0xec => {}
            0xed => {}
            0xee => {
                self.inc(AddressMode::Absolute);
            }
            0xf0 => {}
            0xf1 => {}
            0xf5 => {}
            0xf6 => {
                self.inc(AddressMode::ZeroPageX);
            }
            0xf8 => {
                self.sed();
            }
            0xf9 => {}
            0xfd => {}
            0xfe => {
                self.inc(AddressMode::AbsoluteX);
            }
            _ => {}
        }
    }

    /// Gets an address for an instruction based on an addressing mode
    fn get_address(&self, address_mode: AddressMode) -> u16 {
        let mut addr: u16 = 0;
        match address_mode {
            AddressMode::Immediate => {
                addr = self.reg.pc;
            }
            AddressMode::ZeroPage => {
                addr = self.read_mem(self.reg.pc) as u16;
            }
            AddressMode::ZeroPageX => {
                addr = self.read_mem(self.reg.pc).wrapping_add(self.reg.x) as u16;
            }
            AddressMode::ZeroPageY => {
                addr = self.read_mem(self.reg.pc).wrapping_add(self.reg.y) as u16;
            }
            AddressMode::Absolute => {
                addr = self.read_mem_u16(self.reg.pc);
            }
            AddressMode::AbsoluteX => {
                addr = self
                    .read_mem_u16(self.reg.pc)
                    .wrapping_add(self.reg.x as u16);
            }
            AddressMode::AbsoluteY => {
                addr = self
                    .read_mem_u16(self.reg.pc)
                    .wrapping_add(self.reg.y as u16);
            }
            AddressMode::IndirectX => {
                let base = self.read_mem(self.reg.pc);
                let ptr: u8 = (base as u8).wrapping_add(self.reg.x);
                let lo = self.read_mem(ptr as u16);
                let hi = self.read_mem(ptr.wrapping_add(1) as u16);
                addr = (hi as u16) << 8 | (lo as u16)
            }
            AddressMode::IndirectY => {
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
            self.reg.s |= flag_bit;
        } else {
            self.reg.s &= !flag_bit;
        }
    }

    fn and(&mut self, address_mode: AddressMode) {
        let address = self.get_address(address_mode);
        self.reg.a &= self.read_mem(address);
        self.reg.pc += 1;
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

    fn inc(&mut self, address_mode: AddressMode) {
        let address = self.get_address(address_mode); 
        let value = self.read_mem(address).wrapping_add(1);
        self.reg.pc += 1;
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
        self.reg.pc += 1;
        self.update_flag(1, self.reg.a == 0);
        self.update_flag(7, self.reg.a & 0b1000_0000 != 0);
    }

    fn ldx(&mut self, address_mode: AddressMode) {
        let address = self.get_address(address_mode);
        self.reg.x = self.read_mem(address);
        self.reg.pc += 1;
        self.update_flag(1, self.reg.x == 0);
        self.update_flag(7, self.reg.x & 0b1000_0000 != 0);
    }

    fn ldy(&mut self, address_mode: AddressMode) {
        let address = self.get_address(address_mode);
        self.reg.y = self.read_mem(address);
        self.reg.pc += 1;
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

    fn tax(&mut self) {
        self.reg.x = self.reg.a;
        self.update_flag(1, self.reg.x == 0);
        self.update_flag(7, self.reg.x & 0b1000_0000 != 0);
    }
}
