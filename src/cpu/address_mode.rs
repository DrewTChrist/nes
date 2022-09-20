/// Opcode addressing modes
#[derive(Copy, Clone)]
pub enum AddressMode {
    Immediate(u16),
    ZeroPage(u16),
    ZeroPageX(u16),
    ZeroPageY(u16),
    Absolute(u16),
    AbsoluteX(u16),
    AbsoluteY(u16),
    Accumulator,
    Indirect(u16),
    IndirectX(u16),
    IndirectY(u16),
}

impl AddressMode {
    pub const IMMEDIATE: AddressMode = AddressMode::Immediate(1);
    pub const ZERO_PAGE: AddressMode = AddressMode::ZeroPage(1);
    pub const ZERO_PAGE_X: AddressMode = AddressMode::ZeroPageX(1);
    pub const ZERO_PAGE_Y: AddressMode = AddressMode::ZeroPageY(1);
    pub const ABSOLUTE: AddressMode = AddressMode::Absolute(2);
    pub const ABSOLUTE_X: AddressMode = AddressMode::AbsoluteX(2);
    pub const ABSOLUTE_Y: AddressMode = AddressMode::AbsoluteY(2);
    pub const ACCUMULATOR: AddressMode = AddressMode::Absolute(0);
    pub const INDIRECT: AddressMode = AddressMode::Indirect(2);
    pub const INDIRECT_X: AddressMode = AddressMode::IndirectX(1);
    pub const INDIRECT_Y: AddressMode = AddressMode::IndirectY(1);

    pub fn get_pc_increment(&self) -> u16 {
        match self {
            AddressMode::Immediate(n) => *n,
            AddressMode::ZeroPage(n) => *n,
            AddressMode::ZeroPageX(n) => *n,
            AddressMode::ZeroPageY(n) => *n,
            AddressMode::Absolute(n) => *n,
            AddressMode::AbsoluteX(n) => *n,
            AddressMode::AbsoluteY(n) => *n,
            AddressMode::Indirect(n) => *n,
            AddressMode::IndirectX(n) => *n,
            AddressMode::IndirectY(n) => *n,
            AddressMode::Accumulator => 0,
        }
    }
}
