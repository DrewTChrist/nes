const MEMORY_SIZE: usize = 0xFFFF;
const PROGRAM_ROM: usize = 0x8000;
const STACK_SIZE: usize = 0xFF;
const STACK_BEG: usize = 0x01FF;
const STACK_END: usize = 0x0100;

/// Does twos complement on a value
fn twos_complement(value: u8) -> u8 {
    !value + 1
}

/// Checks if the 8th bit of a u8
/// is on mostly for readability
fn is_negative(value: u8) -> bool {
    value & 0x80 == 0x80
}

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

    fn update_flag(&mut self, flag: u8, condition: bool) {
        let flag_bit: u8 = 1 << flag;
        if condition {
            self.p |= flag_bit;
        } else {
            self.p &= !flag_bit;
        }
    }

    fn enable_flag(&mut self, flag: Flag) {
        match flag {
            Flag::Carry => self.update_flag(0, true),
            Flag::Zero => self.update_flag(1, true),
            Flag::InterruptDisable => self.update_flag(2, true),
            Flag::DecimalMode => self.update_flag(3, true),
            Flag::Overflow => self.update_flag(6, true),
            Flag::Negative => self.update_flag(7, true),
        }
    }

    fn disable_flag(&mut self, flag: Flag) {
        match flag {
            Flag::Carry => self.update_flag(0, false),
            Flag::Zero => self.update_flag(1, false),
            Flag::InterruptDisable => self.update_flag(2, false),
            Flag::DecimalMode => self.update_flag(3, false),
            Flag::Overflow => self.update_flag(6, false),
            Flag::Negative => self.update_flag(7, false),
        }
    }
}

enum Flag {
    Carry = 0,
    Zero = 1,
    InterruptDisable = 2,
    DecimalMode = 3,
    Overflow = 6,
    Negative = 7,
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
        self.read_mem(self.reg.s as u16)
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
            0x10 => self.bpl(),
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
            0x30 => self.bmi(),
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
            0x50 => self.bvc(),
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
            0x70 => self.bvs(),
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
            0x8a => self.txa(),
            0x8c => todo!(),
            0x8d => todo!(),
            0x8e => todo!(),
            0x90 => self.bcc(),
            0x91 => todo!(),
            0x94 => todo!(),
            0x95 => todo!(),
            0x96 => todo!(),
            0x98 => self.tya(),
            0x99 => todo!(),
            0x9a => self.txs(),
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
            0xb0 => self.bcs(),
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
            0xc1 => self.cmp(AddressMode::INDIRECT_X),
            0xc4 => todo!(),
            0xc5 => self.cmp(AddressMode::ZERO_PAGE),
            0xc6 => self.dec(AddressMode::ZERO_PAGE),
            0xc8 => self.iny(),
            0xc9 => self.cmp(AddressMode::IMMEDIATE),
            0xca => self.dex(),
            0xcc => todo!(),
            0xcd => self.cmp(AddressMode::ABSOLUTE),
            0xce => self.dec(AddressMode::ABSOLUTE),
            0xd0 => self.bne(),
            0xd1 => self.cmp(AddressMode::INDIRECT_Y),
            0xd5 => self.cmp(AddressMode::ZERO_PAGE_X),
            0xd6 => self.dec(AddressMode::ZERO_PAGE_X),
            0xd9 => self.cmp(AddressMode::ABSOLUTE_Y),
            0xdd => self.cmp(AddressMode::ABSOLUTE_X),
            0xde => self.dec(AddressMode::ABSOLUTE_X),
            0xd8 => self.cld(),
            0xe0 => self.cpx(AddressMode::IMMEDIATE),
            0xe1 => todo!(),
            0xe4 => self.cpx(AddressMode::ZERO_PAGE),
            0xe5 => todo!(),
            0xe6 => self.inc(AddressMode::ZERO_PAGE),
            0xe8 => self.inx(),
            0xe9 => todo!(),
            0xea => self.nop(),
            0xec => self.cpx(AddressMode::ABSOLUTE),
            0xed => todo!(),
            0xee => self.inc(AddressMode::ABSOLUTE),
            0xf0 => self.beq(),
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
        match address_mode {
            AddressMode::Immediate(_) => self.reg.pc,
            AddressMode::ZeroPage(_) => self.read_mem(self.reg.pc) as u16,
            AddressMode::ZeroPageX(_) => self.read_mem(self.reg.pc).wrapping_add(self.reg.x) as u16,
            AddressMode::ZeroPageY(_) => self.read_mem(self.reg.pc).wrapping_add(self.reg.y) as u16,
            AddressMode::Absolute(_) => self.read_mem_u16(self.reg.pc),
            AddressMode::AbsoluteX(_) => self
                .read_mem_u16(self.reg.pc)
                .wrapping_add(self.reg.x as u16),
            AddressMode::AbsoluteY(_) => self
                .read_mem_u16(self.reg.pc)
                .wrapping_add(self.reg.y as u16),
            AddressMode::IndirectX(_) => {
                let base = self.read_mem(self.reg.pc);
                let ptr: u8 = (base as u8).wrapping_add(self.reg.x);
                let lo = self.read_mem(ptr as u16);
                let hi = self.read_mem(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressMode::IndirectY(_) => {
                let base = self.read_mem(self.reg.pc);
                let lo = self.read_mem(base as u16);
                let hi = self.read_mem((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                deref_base.wrapping_add(self.reg.y as u16)
            }
        }
    }

    fn adc(&mut self, address_mode: AddressMode) {}

    fn and(&mut self, address_mode: AddressMode) {
        let address = self.get_address(address_mode);
        self.reg.a &= self.read_mem(address);
        self.reg.pc += address_mode.get_pc_increment();
        if self.reg.a == 0 {
            self.reg.enable_flag(Flag::Zero);
        }
        if is_negative(self.reg.a) {
            self.reg.enable_flag(Flag::Negative);
        }
    }

    fn asl(&mut self, address_mode: AddressMode) {}

    fn bcc(&mut self) {
        if self.reg.p & 0b0000_0001 == 0 {
            let offset = self.read_mem(self.reg.pc);
            self.reg.pc += 1;
            if is_negative(offset) {
                self.reg.pc -= twos_complement(offset) as u16;
            } else {
                self.reg.pc += offset as u16;
            }
        }
    }

    fn bcs(&mut self) {
        if self.reg.p & 0b0000_0001 != 0 {
            let offset = self.read_mem(self.reg.pc);
            self.reg.pc += 1;
            if is_negative(offset) {
                self.reg.pc -= twos_complement(offset) as u16;
            } else {
                self.reg.pc += offset as u16;
            }
        }
    }

    fn beq(&mut self) {
        if self.reg.p & 0b0000_0010 != 0 {
            let offset = self.read_mem(self.reg.pc);
            self.reg.pc += 1;
            if is_negative(offset) {
                self.reg.pc -= twos_complement(offset) as u16;
            } else {
                self.reg.pc += offset as u16;
            }
        }
    }

    fn bit(&mut self, address_mode: AddressMode) {}

    fn bmi(&mut self) {
        if self.reg.p & 0b1000_0000 != 0 {
            let offset = self.read_mem(self.reg.pc);
            self.reg.pc += 1;
            if is_negative(offset) {
                self.reg.pc -= twos_complement(offset) as u16;
            } else {
                self.reg.pc += offset as u16;
            }
        }
    }

    fn bne(&mut self) {
        if self.reg.p & 0b0000_0010 == 0 {
            let offset = self.read_mem(self.reg.pc);
            self.reg.pc += 1;
            if is_negative(offset) {
                self.reg.pc -= twos_complement(offset) as u16;
            } else {
                self.reg.pc += offset as u16;
            }
        }
    }

    fn bpl(&mut self) {
        if self.reg.p & 0b1000_0000 == 0 {
            let offset = self.read_mem(self.reg.pc);
            self.reg.pc += 1;
            if is_negative(offset) {
                self.reg.pc -= twos_complement(offset) as u16;
            } else {
                self.reg.pc += offset as u16;
            }
        }
    }

    fn brk(&mut self) {}

    fn bvc(&mut self) {
        if self.reg.p & 0b0100_0000 == 0 {
            let offset = self.read_mem(self.reg.pc);
            self.reg.pc += 1;
            if is_negative(offset) {
                self.reg.pc -= twos_complement(offset) as u16;
            } else {
                self.reg.pc += offset as u16;
            }
        }
    }

    fn bvs(&mut self) {
        if self.reg.p & 0b0100_0000 != 0 {
            let offset = self.read_mem(self.reg.pc);
            self.reg.pc += 1;
            if is_negative(offset) {
                self.reg.pc -= twos_complement(offset) as u16;
            } else {
                self.reg.pc += offset as u16;
            }
        }
    }

    fn clc(&mut self) {
        self.reg.disable_flag(Flag::Carry);
    }

    fn cld(&mut self) {
        self.reg.disable_flag(Flag::DecimalMode);
    }

    fn cli(&mut self) {
        self.reg.disable_flag(Flag::InterruptDisable);
    }

    fn clv(&mut self) {
        self.reg.disable_flag(Flag::Overflow);
    }

    fn cmp(&mut self, address_mode: AddressMode) {
        let address = self.get_address(address_mode);
        let value = self.read_mem(address);
        self.reg.pc += address_mode.get_pc_increment();
        if self.reg.a >= value {
            self.reg.enable_flag(Flag::Carry);
        }
        if self.reg.a == value {
            self.reg.enable_flag(Flag::Zero);
        }
        if is_negative(self.reg.a.wrapping_sub(value)) {
            self.reg.enable_flag(Flag::Negative);
        }
    }

    fn cpx(&mut self, address_mode: AddressMode) {
        let address = self.get_address(address_mode);
        let value = self.read_mem(address);
        self.reg.pc += address_mode.get_pc_increment();
        if self.reg.x >= value {
            self.reg.enable_flag(Flag::Carry);
        }
        if self.reg.x == value {
            self.reg.enable_flag(Flag::Zero);
        }
        if is_negative(self.reg.x.wrapping_sub(value)) {
            self.reg.enable_flag(Flag::Negative);
        }
    }

    fn cpy(&mut self, address_mode: AddressMode) {}

    fn dec(&mut self, address_mode: AddressMode) {
        let address = self.get_address(address_mode);
        let value = self.read_mem(address).wrapping_sub(1);
        self.reg.pc += address_mode.get_pc_increment();
        self.write_mem(address, value);
        if value == 0 {
            self.reg.enable_flag(Flag::Zero);
        }
        if is_negative(value) {
            self.reg.enable_flag(Flag::Negative);
        }
    }

    fn dex(&mut self) {
        self.reg.x = self.reg.x.wrapping_sub(1);
        if self.reg.x == 0 {
            self.reg.enable_flag(Flag::Zero);
        }
        if is_negative(self.reg.x) {
            self.reg.enable_flag(Flag::Negative);
        }
    }

    fn dey(&mut self) {
        self.reg.y = self.reg.y.wrapping_sub(1);
        if self.reg.y == 0 {
            self.reg.enable_flag(Flag::Zero);
        }
        if is_negative(self.reg.y) {
            self.reg.enable_flag(Flag::Negative);
        }
    }

    fn eor(&mut self, address_mode: AddressMode) {}

    fn inc(&mut self, address_mode: AddressMode) {
        let address = self.get_address(address_mode);
        let value = self.read_mem(address).wrapping_add(1);
        self.reg.pc += address_mode.get_pc_increment();
        self.write_mem(address, value);
        if value == 0 {
            self.reg.enable_flag(Flag::Zero);
        }
        if is_negative(value) {
            self.reg.enable_flag(Flag::Negative);
        }
    }

    fn inx(&mut self) {
        self.reg.x = self.reg.x.wrapping_add(1);
        if self.reg.x == 0 {
            self.reg.enable_flag(Flag::Zero);
        }
        if is_negative(self.reg.x) {
            self.reg.enable_flag(Flag::Negative);
        }
    }

    fn iny(&mut self) {
        self.reg.y = self.reg.y.wrapping_add(1);
        if self.reg.y == 0 {
            self.reg.enable_flag(Flag::Zero);
        }
        if is_negative(self.reg.y) {
            self.reg.enable_flag(Flag::Negative);
        }
    }

    fn jmp(&mut self, address_mode: AddressMode) {}

    fn jsr(&mut self, address_mode: AddressMode) {}

    fn lda(&mut self, address_mode: AddressMode) {
        let address = self.get_address(address_mode);
        self.reg.a = self.read_mem(address);
        self.reg.pc += address_mode.get_pc_increment();
        if self.reg.a == 0 {
            self.reg.enable_flag(Flag::Zero);
        }
        if is_negative(self.reg.a) {
            self.reg.enable_flag(Flag::Negative);
        }
    }

    fn ldx(&mut self, address_mode: AddressMode) {
        let address = self.get_address(address_mode);
        self.reg.x = self.read_mem(address);
        self.reg.pc += address_mode.get_pc_increment();
        if self.reg.x == 0 {
            self.reg.enable_flag(Flag::Zero);
        }
        if is_negative(self.reg.x) {
            self.reg.enable_flag(Flag::Negative);
        }
    }

    fn ldy(&mut self, address_mode: AddressMode) {
        let address = self.get_address(address_mode);
        self.reg.y = self.read_mem(address);
        self.reg.pc += address_mode.get_pc_increment();
        if self.reg.y == 0 {
            self.reg.enable_flag(Flag::Zero);
        }
        if is_negative(self.reg.y) {
            self.reg.enable_flag(Flag::Negative);
        }
    }

    fn lsr(&mut self, address_mode: AddressMode) {}

    fn nop(&self) {}

    fn ora(&mut self, address_mode: AddressMode) {}

    fn pha(&mut self) {}

    fn php(&mut self) {}

    fn pla(&mut self) {}

    fn plp(&mut self) {}

    fn rol(&mut self, address_mode: AddressMode) {}

    fn ror(&mut self, address_mode: AddressMode) {}

    fn rti(&mut self) {}

    fn rts(&mut self) {}

    fn sbc(&mut self, address_mode: AddressMode) {}

    fn sec(&mut self) {
        self.reg.enable_flag(Flag::Carry);
    }

    fn sed(&mut self) {
        self.reg.enable_flag(Flag::DecimalMode);
    }

    fn sei(&mut self) {
        self.reg.enable_flag(Flag::InterruptDisable);
    }

    fn sta(&mut self, address_mode: AddressMode) {}

    fn stx(&mut self, address_mode: AddressMode) {}

    fn sty(&mut self, address_mode: AddressMode) {}

    fn tax(&mut self) {
        self.reg.x = self.reg.a;
        if self.reg.x == 0 {
            self.reg.enable_flag(Flag::Zero);
        }
        if is_negative(self.reg.x) {
            self.reg.enable_flag(Flag::Negative);
        }
    }

    fn tay(&mut self) {
        self.reg.y = self.reg.a;
        if self.reg.y == 0 {
            self.reg.enable_flag(Flag::Zero);
        }
        if is_negative(self.reg.y) {
            self.reg.enable_flag(Flag::Negative);
        }
    }

    fn tsx(&mut self) {
        self.reg.x = self.reg.s;
        if self.reg.x == 0 {
            self.reg.enable_flag(Flag::Zero);
        }
        if is_negative(self.reg.x) {
            self.reg.enable_flag(Flag::Negative);
        }
    }

    fn txa(&mut self) {
        self.reg.a = self.reg.x;
        if self.reg.a == 0 {
            self.reg.enable_flag(Flag::Zero);
        }
        if is_negative(self.reg.a) {
            self.reg.enable_flag(Flag::Negative);
        }
    }

    fn txs(&mut self) {
        self.reg.s = self.reg.x;
    }

    fn tya(&mut self) {
        self.reg.a = self.reg.y;
        if self.reg.a == 0 {
            self.reg.enable_flag(Flag::Zero);
        }
        if is_negative(self.reg.a) {
            self.reg.enable_flag(Flag::Negative);
        }
    }
}
