mod address_mode;
mod instructions;
mod registers;

use address_mode::AddressMode;
use instructions::INSTRUCTIONS;
use registers::{Flag, Registers};

const MEMORY_SIZE: usize = 0xFFFF;
const PROGRAM_ROM: usize = 0x8000;
const STACK_SIZE: usize = 0xFF;
const STACK_BEG: usize = 0x01FF;
const STACK_END: usize = 0x0100;

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
    /// Creates a new cpu struct
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

    /// Returns the cpu stack as a slice
    pub fn get_stack(&self) -> &[u8] {
        self.get_mem_slice(STACK_END, STACK_BEG + 1)
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

    /// Pushes a byte onto the cpu stack
    pub fn push_stack(&mut self, value: u8) {
        self.write_mem(STACK_END as u16 | self.reg.s as u16, value);
        self.reg.s = self.reg.s.wrapping_sub(1);
    }

    /// Pushes 16 bit value onto the cpu stack
    pub fn push_stack_u16(&mut self, value: u16) {
        self.write_mem_u16(STACK_END as u16 | self.reg.s as u16, value);
        self.reg.s = self.reg.s.wrapping_sub(2);
    }

    /// Pops a byte from the cpu stack
    pub fn pop_stack(&mut self) -> u8 {
        self.reg.s = self.reg.s.wrapping_add(1);
        self.read_mem(STACK_END as u16 | self.reg.s as u16)
    }

    /// Pops a 16 bit value from the cpu stack
    pub fn pop_stack_u16(&mut self) -> u16 {
        self.reg.s = self.reg.s.wrapping_add(2);
        self.read_mem_u16(STACK_END as u16 | self.reg.s as u16)
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
        let func = INSTRUCTIONS[opcode as usize].0;
        let address_mode = INSTRUCTIONS[opcode as usize].1;
        func(self, address_mode);
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
            AddressMode::Indirect(_) => {
                if self.reg.pc & 0xff == 0xff {
                    let lo = self.read_mem(self.reg.pc);
                    let hi = self.read_mem(self.reg.pc & 0xff00);
                    (hi as u16) << 8 | (lo as u16)
                } else {
                    self.get_address(AddressMode::ABSOLUTE)
                }
            }
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
            // Accumulator addressing mode does not need an address
            AddressMode::Accumulator => 0,
            AddressMode::NoMode => 0,
        }
    }
}
