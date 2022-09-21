use crate::cpu::PROGRAM_ROM;
use crate::cpu::STACK_BEG;

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

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: PROGRAM_ROM as u16,
            s: 0xff,
            p: 0,
        }
    }

    pub fn reset(&mut self, pc: u16) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.pc = pc;
        self.s = 0;
        self.p = 0;
    }

    pub fn update_flag(&mut self, flag: u8, condition: bool) {
        let flag_bit: u8 = 1 << flag;
        if condition {
            self.p |= flag_bit;
        } else {
            self.p &= !flag_bit;
        }
    }

    pub fn enable_flag(&mut self, flag: Flag) {
        match flag {
            Flag::Carry => self.update_flag(Flag::Carry as u8, true),
            Flag::Zero => self.update_flag(Flag::Zero as u8, true),
            Flag::InterruptDisable => self.update_flag(Flag::InterruptDisable as u8, true),
            Flag::DecimalMode => self.update_flag(Flag::DecimalMode as u8, true),
            Flag::Break => self.update_flag(Flag::Break as u8, true),
            Flag::Overflow => self.update_flag(Flag::Overflow as u8, true),
            Flag::Negative => self.update_flag(Flag::Negative as u8, true),
        }
    }

    pub fn disable_flag(&mut self, flag: Flag) {
        match flag {
            Flag::Carry => self.update_flag(Flag::Carry as u8, false),
            Flag::Zero => self.update_flag(Flag::Zero as u8, false),
            Flag::InterruptDisable => self.update_flag(Flag::InterruptDisable as u8, false),
            Flag::DecimalMode => self.update_flag(Flag::DecimalMode as u8, false),
            Flag::Break => self.update_flag(Flag::Break as u8, false),
            Flag::Overflow => self.update_flag(Flag::Overflow as u8, false),
            Flag::Negative => self.update_flag(Flag::Negative as u8, false),
        }
    }
}

pub enum Flag {
    Carry = 0,
    Zero = 1,
    InterruptDisable = 2,
    DecimalMode = 3,
    Break = 5,
    Overflow = 6,
    Negative = 7,
}
