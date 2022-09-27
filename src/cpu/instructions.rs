use crate::cpu::{AddressMode, Cpu, Flag};

/// Does twos complement on a value
fn twos_complement(value: u8) -> u8 {
    !value + 1
}

/// Checks if the 8th bit of a u8
/// is on mostly for readability
fn is_negative(value: u8) -> bool {
    value & 0x80 == 0x80
}

/// Replaces a bit at a location with another specified bit
fn replace_bit(number: &mut u8, position: u8, value: u8) {
    let mask: u8 = 0b1 << position;
    *number = (*number & !mask) | (value << position);
}

fn adc(cpu: &mut Cpu, address_mode: AddressMode) {}

fn and(cpu: &mut Cpu, address_mode: AddressMode) {
    let address = cpu.get_address(address_mode);
    cpu.reg.a &= cpu.read_mem(address);
    cpu.reg.pc += address_mode.get_pc_increment();
    if cpu.reg.a == 0 {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(cpu.reg.a) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn asl(cpu: &mut Cpu, address_mode: AddressMode) {
    if let AddressMode::Accumulator = address_mode {
        let bit_7 = (cpu.reg.a & 0b1000_0000) >> 7;
        cpu.reg.a <<= 1;
        replace_bit(&mut cpu.reg.p, 0, bit_7);
        if cpu.reg.a == 0 {
            cpu.reg.enable_flag(Flag::Zero);
        }
        if cpu.reg.a & 0x80 == 0x80 {
            cpu.reg.enable_flag(Flag::Negative);
        }
    } else {
        let address = cpu.get_address(address_mode);
        let mut value = cpu.read_mem(address);
        let bit_7 = (value & 0b1000_0000) >> 7;
        value <<= 1;
        cpu.write_mem(address, value);
        replace_bit(&mut cpu.reg.p, 0, bit_7);
        if value & 0x80 == 0x80 {
            cpu.reg.enable_flag(Flag::Negative);
        }
    }
}

fn bcc(cpu: &mut Cpu, _: AddressMode) {
    if cpu.reg.p & 0b0000_0001 == 0 {
        let offset = cpu.read_mem(cpu.reg.pc);
        if is_negative(offset) {
            cpu.reg.pc -= twos_complement(offset) as u16;
        } else {
            cpu.reg.pc += offset as u16;
        }
    }
    cpu.reg.pc += 1;
}

fn bcs(cpu: &mut Cpu, _: AddressMode) {
    if cpu.reg.p & 0b0000_0001 != 0 {
        let offset = cpu.read_mem(cpu.reg.pc);
        if is_negative(offset) {
            cpu.reg.pc -= twos_complement(offset) as u16;
        } else {
            cpu.reg.pc += offset as u16;
        }
    }
    cpu.reg.pc += 1;
}

fn beq(cpu: &mut Cpu, _: AddressMode) {
    if cpu.reg.p & 0b0000_0010 != 0 {
        let offset = cpu.read_mem(cpu.reg.pc);
        if is_negative(offset) {
            cpu.reg.pc -= twos_complement(offset) as u16;
        } else {
            cpu.reg.pc += offset as u16;
        }
    }
    cpu.reg.pc += 1;
}

fn bit(cpu: &mut Cpu, address_mode: AddressMode) {
    let address = cpu.get_address(address_mode);
    let value = cpu.read_mem(address);
    let result = value & cpu.reg.a;
    if result == 0 {
        cpu.reg.enable_flag(Flag::Zero);
    }
    cpu.reg.p |= result & 0x80;
    cpu.reg.p |= result & 0x40;
}

fn bmi(cpu: &mut Cpu, _: AddressMode) {
    if cpu.reg.p & 0b1000_0000 != 0 {
        let offset = cpu.read_mem(cpu.reg.pc);
        if is_negative(offset) {
            cpu.reg.pc -= twos_complement(offset) as u16;
        } else {
            cpu.reg.pc += offset as u16;
        }
    }
    cpu.reg.pc += 1;
}

fn bne(cpu: &mut Cpu, _: AddressMode) {
    if cpu.reg.p & 0b0000_0010 == 0 {
        let offset = cpu.read_mem(cpu.reg.pc);
        if is_negative(offset) {
            cpu.reg.pc -= twos_complement(offset) as u16;
        } else {
            cpu.reg.pc += offset as u16;
        }
    }
    cpu.reg.pc += 1;
}

fn bpl(cpu: &mut Cpu, _: AddressMode) {
    if cpu.reg.p & 0b1000_0000 == 0 {
        let offset = cpu.read_mem(cpu.reg.pc);
        if is_negative(offset) {
            cpu.reg.pc -= twos_complement(offset) as u16;
        } else {
            cpu.reg.pc += offset as u16;
        }
    }
    cpu.reg.pc += 1;
}

fn brk(cpu: &mut Cpu, _: AddressMode) {
    cpu.push_stack_u16(cpu.reg.pc);
    cpu.push_stack(cpu.reg.p);
    cpu.reg.pc = cpu.read_mem_u16(0xfffd);
    cpu.reg.enable_flag(Flag::Break);
}

fn bvc(cpu: &mut Cpu, _: AddressMode) {
    if cpu.reg.p & 0b0100_0000 == 0 {
        let offset = cpu.read_mem(cpu.reg.pc);
        if is_negative(offset) {
            cpu.reg.pc -= twos_complement(offset) as u16;
        } else {
            cpu.reg.pc += offset as u16;
        }
    }
    cpu.reg.pc += 1;
}

fn bvs(cpu: &mut Cpu, _: AddressMode) {
    if cpu.reg.p & 0b0100_0000 != 0 {
        let offset = cpu.read_mem(cpu.reg.pc);
        if is_negative(offset) {
            cpu.reg.pc -= twos_complement(offset) as u16;
        } else {
            cpu.reg.pc += offset as u16;
        }
    }
    cpu.reg.pc += 1;
}

fn clc(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.disable_flag(Flag::Carry);
}

fn cld(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.disable_flag(Flag::DecimalMode);
}

fn cli(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.disable_flag(Flag::InterruptDisable);
}

fn clv(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.disable_flag(Flag::Overflow);
}

fn cmp(cpu: &mut Cpu, address_mode: AddressMode) {
    let address = cpu.get_address(address_mode);
    let value = cpu.read_mem(address);
    cpu.reg.pc += address_mode.get_pc_increment();
    if cpu.reg.a >= value {
        cpu.reg.enable_flag(Flag::Carry);
    }
    if cpu.reg.a == value {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(cpu.reg.a.wrapping_sub(value)) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn cpx(cpu: &mut Cpu, address_mode: AddressMode) {
    let address = cpu.get_address(address_mode);
    let value = cpu.read_mem(address);
    cpu.reg.pc += address_mode.get_pc_increment();
    if cpu.reg.x >= value {
        cpu.reg.enable_flag(Flag::Carry);
    }
    if cpu.reg.x == value {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(cpu.reg.x.wrapping_sub(value)) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn cpy(cpu: &mut Cpu, address_mode: AddressMode) {
    let address = cpu.get_address(address_mode);
    let value = cpu.read_mem(address);
    cpu.reg.pc += address_mode.get_pc_increment();
    if cpu.reg.y >= value {
        cpu.reg.enable_flag(Flag::Carry);
    }
    if cpu.reg.y == value {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(cpu.reg.y.wrapping_sub(value)) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn dec(cpu: &mut Cpu, address_mode: AddressMode) {
    let address = cpu.get_address(address_mode);
    let value = cpu.read_mem(address).wrapping_sub(1);
    cpu.reg.pc += address_mode.get_pc_increment();
    cpu.write_mem(address, value);
    if value == 0 {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(value) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn dex(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.x = cpu.reg.x.wrapping_sub(1);
    if cpu.reg.x == 0 {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(cpu.reg.x) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn dey(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.y = cpu.reg.y.wrapping_sub(1);
    if cpu.reg.y == 0 {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(cpu.reg.y) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn eor(cpu: &mut Cpu, address_mode: AddressMode) {
    let address = cpu.get_address(address_mode);
    let value = cpu.read_mem(address);
    cpu.reg.pc += address_mode.get_pc_increment();
    cpu.reg.a ^= value;
    if cpu.reg.a == 0 {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(cpu.reg.a) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn inc(cpu: &mut Cpu, address_mode: AddressMode) {
    let address = cpu.get_address(address_mode);
    let value = cpu.read_mem(address).wrapping_add(1);
    cpu.reg.pc += address_mode.get_pc_increment();
    cpu.write_mem(address, value);
    if value == 0 {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(value) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn inx(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.x = cpu.reg.x.wrapping_add(1);
    if cpu.reg.x == 0 {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(cpu.reg.x) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn iny(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.y = cpu.reg.y.wrapping_add(1);
    if cpu.reg.y == 0 {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(cpu.reg.y) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn jmp(cpu: &mut Cpu, address_mode: AddressMode) {
    cpu.reg.pc = cpu.get_address(address_mode);
}

fn jsr(cpu: &mut Cpu, address_mode: AddressMode) {
    let address = cpu.get_address(address_mode);
    cpu.reg.pc += address_mode.get_pc_increment();
    cpu.push_stack_u16(cpu.reg.pc - 1);
    cpu.reg.pc = address;
}

fn lda(cpu: &mut Cpu, address_mode: AddressMode) {
    let address = cpu.get_address(address_mode);
    cpu.reg.a = cpu.read_mem(address);
    cpu.reg.pc += address_mode.get_pc_increment();
    if cpu.reg.a == 0 {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(cpu.reg.a) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn ldx(cpu: &mut Cpu, address_mode: AddressMode) {
    let address = cpu.get_address(address_mode);
    cpu.reg.x = cpu.read_mem(address);
    cpu.reg.pc += address_mode.get_pc_increment();
    if cpu.reg.x == 0 {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(cpu.reg.x) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn ldy(cpu: &mut Cpu, address_mode: AddressMode) {
    let address = cpu.get_address(address_mode);
    cpu.reg.y = cpu.read_mem(address);
    cpu.reg.pc += address_mode.get_pc_increment();
    if cpu.reg.y == 0 {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(cpu.reg.y) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn lsr(cpu: &mut Cpu, address_mode: AddressMode) {
    if let AddressMode::Accumulator = address_mode {
        let bit_0 = cpu.reg.a & 0b1;
        cpu.reg.a >>= 1;
        replace_bit(&mut cpu.reg.p, 0, bit_0);
        if cpu.reg.a == 0 {
            cpu.reg.enable_flag(Flag::Zero);
        }
        if cpu.reg.a & 0x80 == 0x80 {
            cpu.reg.enable_flag(Flag::Negative);
        }
    } else {
        let address = cpu.get_address(address_mode);
        let mut value = cpu.read_mem(address);
        let bit_0 = value & 0b1;
        value >>= 1;
        cpu.write_mem(address, value);
        replace_bit(&mut cpu.reg.p, 0, bit_0);
        if value == 0 {
            cpu.reg.enable_flag(Flag::Negative);
        }
        if value & 0x80 == 0x80 {
            cpu.reg.enable_flag(Flag::Negative);
        }
    }
}

fn nop(cpu: &mut Cpu, _: AddressMode) {
    /* no operation */
}

fn ora(cpu: &mut Cpu, address_mode: AddressMode) {
    let address = cpu.get_address(address_mode);
    let value = cpu.read_mem(address);
    cpu.reg.pc += address_mode.get_pc_increment();
    cpu.reg.a |= value;
    if cpu.reg.a == 0 {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(cpu.reg.a) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn pha(cpu: &mut Cpu, _: AddressMode) {
    cpu.push_stack(cpu.reg.a);
}

fn php(cpu: &mut Cpu, _: AddressMode) {
    cpu.push_stack(cpu.reg.p);
}

fn pla(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.a = cpu.pop_stack();
}

fn plp(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.p = cpu.pop_stack();
}

fn rol(cpu: &mut Cpu, address_mode: AddressMode) {
    if let AddressMode::Accumulator = address_mode {
        let bit_7 = (cpu.reg.a & 0x80) >> 7;
        let carry = cpu.reg.p & 0x1;
        cpu.reg.a <<= 1;
        cpu.reg.a |= carry;
        replace_bit(&mut cpu.reg.p, 0, bit_7);
        if cpu.reg.a == 0 {
            cpu.reg.enable_flag(Flag::Zero);
        }
        if is_negative(cpu.reg.a) {
            cpu.reg.enable_flag(Flag::Negative);
        }
    } else {
        let address = cpu.get_address(address_mode);
        cpu.reg.pc += address_mode.get_pc_increment();
        let mut value = cpu.read_mem(address);
        let bit_7 = (value & 0x80) >> 7;
        let carry = cpu.reg.p & 0x1;
        value <<= 1;
        value |= carry;
        cpu.write_mem(address, value);
        replace_bit(&mut cpu.reg.p, 0, bit_7);
        if is_negative(value) {
            cpu.reg.enable_flag(Flag::Negative);
        }
    }
}

fn ror(cpu: &mut Cpu, address_mode: AddressMode) {
    if let AddressMode::Accumulator = address_mode {
        let bit_0 = cpu.reg.a & 0x1;
        let carry = (cpu.reg.p & 0x1) << 7;
        cpu.reg.a >>= 1;
        cpu.reg.a |= carry;
        replace_bit(&mut cpu.reg.p, 0, bit_0);
        if cpu.reg.a == 0 {
            cpu.reg.enable_flag(Flag::Zero);
        }
        if is_negative(cpu.reg.a) {
            cpu.reg.enable_flag(Flag::Negative);
        }
    } else {
        let address = cpu.get_address(address_mode);
        cpu.reg.pc += address_mode.get_pc_increment();
        let mut value = cpu.read_mem(address);
        let bit_0 = value & 0x1;
        let carry = (cpu.reg.p & 0x1) << 7;
        value >>= 1;
        value |= carry;
        cpu.write_mem(address, value);
        replace_bit(&mut cpu.reg.p, 0, bit_0);
        if is_negative(value) {
            cpu.reg.enable_flag(Flag::Negative);
        }
    }
}

fn rti(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.p = cpu.pop_stack();
    cpu.reg.pc = cpu.pop_stack_u16();
}

fn rts(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.pc = cpu.pop_stack_u16() + 1;
}

fn sbc(cpu: &mut Cpu, address_mode: AddressMode) {}

fn sec(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.enable_flag(Flag::Carry);
}

fn sed(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.enable_flag(Flag::DecimalMode);
}

fn sei(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.enable_flag(Flag::InterruptDisable);
}

fn sta(cpu: &mut Cpu, address_mode: AddressMode) {
    let address = cpu.get_address(address_mode);
    cpu.reg.pc += address_mode.get_pc_increment();
    cpu.write_mem(address, cpu.reg.a);
}

fn stx(cpu: &mut Cpu, address_mode: AddressMode) {
    let address = cpu.get_address(address_mode);
    cpu.reg.pc += address_mode.get_pc_increment();
    cpu.write_mem(address, cpu.reg.x);
}

fn sty(cpu: &mut Cpu, address_mode: AddressMode) {
    let address = cpu.get_address(address_mode);
    cpu.reg.pc += address_mode.get_pc_increment();
    cpu.write_mem(address, cpu.reg.y);
}

fn tax(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.x = cpu.reg.a;
    if cpu.reg.x == 0 {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(cpu.reg.x) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn tay(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.y = cpu.reg.a;
    if cpu.reg.y == 0 {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(cpu.reg.y) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn tsx(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.x = cpu.reg.s;
    if cpu.reg.x == 0 {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(cpu.reg.x) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn txa(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.a = cpu.reg.x;
    if cpu.reg.a == 0 {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(cpu.reg.a) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

fn txs(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.s = cpu.reg.x;
}

fn tya(cpu: &mut Cpu, _: AddressMode) {
    cpu.reg.a = cpu.reg.y;
    if cpu.reg.a == 0 {
        cpu.reg.enable_flag(Flag::Zero);
    }
    if is_negative(cpu.reg.a) {
        cpu.reg.enable_flag(Flag::Negative);
    }
}

/// Tuple to hold function pointer and address mode
type Instruction = (fn(&mut Cpu, AddressMode) -> (), AddressMode);

/// This array of instructions should be indexed
/// by the current opcode. The gaps, which shouldn't
/// be indexed have been filled with nops
pub const INSTRUCTIONS: [Instruction; 0x100] = [
    (brk, AddressMode::NoMode),
    (ora, AddressMode::INDIRECT_X),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (ora, AddressMode::ZERO_PAGE),
    (asl, AddressMode::ZERO_PAGE),
    (nop, AddressMode::NoMode),
    (php, AddressMode::NoMode),
    (ora, AddressMode::IMMEDIATE),
    (asl, AddressMode::ACCUMULATOR),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (ora, AddressMode::ABSOLUTE),
    (asl, AddressMode::ABSOLUTE),
    (nop, AddressMode::NoMode),
    (bpl, AddressMode::NoMode),
    (ora, AddressMode::INDIRECT_Y),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (ora, AddressMode::ZERO_PAGE_X),
    (asl, AddressMode::ZERO_PAGE_X),
    (nop, AddressMode::NoMode),
    (clc, AddressMode::NoMode),
    (ora, AddressMode::ABSOLUTE_Y),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (ora, AddressMode::ABSOLUTE_X),
    (asl, AddressMode::ABSOLUTE_X),
    (nop, AddressMode::NoMode),
    (jsr, AddressMode::ABSOLUTE),
    (and, AddressMode::INDIRECT_X),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (bit, AddressMode::ZERO_PAGE),
    (and, AddressMode::ZERO_PAGE_X),
    (rol, AddressMode::ZERO_PAGE),
    (nop, AddressMode::NoMode),
    (plp, AddressMode::NoMode),
    (and, AddressMode::IMMEDIATE),
    (rol, AddressMode::ACCUMULATOR),
    (nop, AddressMode::NoMode),
    (bit, AddressMode::ABSOLUTE),
    (and, AddressMode::ABSOLUTE),
    (rol, AddressMode::ABSOLUTE),
    (nop, AddressMode::NoMode),
    (bmi, AddressMode::NoMode),
    (and, AddressMode::INDIRECT_Y),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (and, AddressMode::ZERO_PAGE_X),
    (rol, AddressMode::ZERO_PAGE_X),
    (nop, AddressMode::NoMode),
    (sec, AddressMode::NoMode),
    (and, AddressMode::ABSOLUTE_Y),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (and, AddressMode::ABSOLUTE_X),
    (rol, AddressMode::ABSOLUTE_X),
    (nop, AddressMode::NoMode),
    (rti, AddressMode::NoMode),
    (eor, AddressMode::INDIRECT_X),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (eor, AddressMode::ZERO_PAGE),
    (lsr, AddressMode::ZERO_PAGE),
    (nop, AddressMode::NoMode),
    (pha, AddressMode::NoMode),
    (eor, AddressMode::IMMEDIATE),
    (lsr, AddressMode::ACCUMULATOR),
    (nop, AddressMode::NoMode),
    (jmp, AddressMode::ABSOLUTE),
    (eor, AddressMode::ABSOLUTE),
    (lsr, AddressMode::ABSOLUTE),
    (nop, AddressMode::NoMode),
    (bvc, AddressMode::NoMode),
    (eor, AddressMode::INDIRECT_Y),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (eor, AddressMode::ZERO_PAGE_X),
    (lsr, AddressMode::ZERO_PAGE_X),
    (nop, AddressMode::NoMode),
    (cli, AddressMode::NoMode),
    (eor, AddressMode::ABSOLUTE_Y),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (eor, AddressMode::ABSOLUTE_X),
    (lsr, AddressMode::ABSOLUTE_X),
    (nop, AddressMode::NoMode),
    (rts, AddressMode::NoMode),
    (adc, AddressMode::INDIRECT_X),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (adc, AddressMode::ZERO_PAGE),
    (ror, AddressMode::ZERO_PAGE),
    (nop, AddressMode::NoMode),
    (pla, AddressMode::NoMode),
    (adc, AddressMode::IMMEDIATE),
    (ror, AddressMode::ACCUMULATOR),
    (nop, AddressMode::NoMode),
    (jmp, AddressMode::INDIRECT),
    (adc, AddressMode::ABSOLUTE),
    (ror, AddressMode::ABSOLUTE),
    (nop, AddressMode::NoMode),
    (bvs, AddressMode::NoMode),
    (adc, AddressMode::INDIRECT_Y),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (adc, AddressMode::ZERO_PAGE_X),
    (ror, AddressMode::ZERO_PAGE_X),
    (nop, AddressMode::NoMode),
    (sei, AddressMode::NoMode),
    (adc, AddressMode::ABSOLUTE_Y),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (adc, AddressMode::ABSOLUTE_X),
    (ror, AddressMode::ABSOLUTE_X),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (sta, AddressMode::INDIRECT_X),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (sty, AddressMode::ZERO_PAGE),
    (sta, AddressMode::ZERO_PAGE),
    (stx, AddressMode::ZERO_PAGE),
    (nop, AddressMode::NoMode),
    (dey, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (txa, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (sty, AddressMode::ABSOLUTE),
    (sta, AddressMode::ABSOLUTE),
    (stx, AddressMode::ABSOLUTE),
    (nop, AddressMode::NoMode),
    (bcc, AddressMode::NoMode),
    (sta, AddressMode::INDIRECT_X),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (sty, AddressMode::ZERO_PAGE_X),
    (sta, AddressMode::ZERO_PAGE_X),
    (stx, AddressMode::ZERO_PAGE_Y),
    (nop, AddressMode::NoMode),
    (tya, AddressMode::NoMode),
    (sta, AddressMode::ABSOLUTE_Y),
    (txs, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (sta, AddressMode::ABSOLUTE_X),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (ldy, AddressMode::IMMEDIATE),
    (lda, AddressMode::INDIRECT_X),
    (ldx, AddressMode::IMMEDIATE),
    (nop, AddressMode::NoMode),
    (ldy, AddressMode::ZERO_PAGE),
    (lda, AddressMode::ZERO_PAGE),
    (ldx, AddressMode::ZERO_PAGE),
    (nop, AddressMode::NoMode),
    (tay, AddressMode::NoMode),
    (lda, AddressMode::IMMEDIATE),
    (tax, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (ldy, AddressMode::ABSOLUTE),
    (lda, AddressMode::ABSOLUTE),
    (ldx, AddressMode::ABSOLUTE),
    (nop, AddressMode::NoMode),
    (bcs, AddressMode::NoMode),
    (lda, AddressMode::INDIRECT_Y),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (ldy, AddressMode::ZERO_PAGE_X),
    (lda, AddressMode::ZERO_PAGE_X),
    (ldx, AddressMode::ZERO_PAGE_X),
    (nop, AddressMode::NoMode),
    (clv, AddressMode::NoMode),
    (lda, AddressMode::ABSOLUTE_Y),
    (tsx, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (ldy, AddressMode::ABSOLUTE_X),
    (lda, AddressMode::ABSOLUTE_X),
    (ldx, AddressMode::ABSOLUTE_X),
    (nop, AddressMode::NoMode),
    (cpy, AddressMode::IMMEDIATE),
    (cmp, AddressMode::INDIRECT_X),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (cpy, AddressMode::ZERO_PAGE),
    (cmp, AddressMode::ZERO_PAGE),
    (dec, AddressMode::ZERO_PAGE),
    (nop, AddressMode::NoMode),
    (iny, AddressMode::NoMode),
    (cmp, AddressMode::IMMEDIATE),
    (dex, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (cpy, AddressMode::ABSOLUTE),
    (cmp, AddressMode::ABSOLUTE),
    (dec, AddressMode::ABSOLUTE),
    (nop, AddressMode::NoMode),
    (bne, AddressMode::NoMode),
    (cmp, AddressMode::INDIRECT_Y),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (cmp, AddressMode::ZERO_PAGE_X),
    (dec, AddressMode::ZERO_PAGE_X),
    (nop, AddressMode::NoMode),
    (cld, AddressMode::NoMode),
    (cmp, AddressMode::ABSOLUTE_Y),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (cmp, AddressMode::ABSOLUTE_X),
    (dec, AddressMode::ABSOLUTE_X),
    (nop, AddressMode::NoMode),
    (cpx, AddressMode::IMMEDIATE),
    (sbc, AddressMode::INDIRECT_X),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (cpx, AddressMode::ZERO_PAGE),
    (sbc, AddressMode::ZERO_PAGE),
    (inc, AddressMode::ZERO_PAGE),
    (nop, AddressMode::NoMode),
    (inx, AddressMode::NoMode),
    (sbc, AddressMode::IMMEDIATE),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (cpx, AddressMode::ABSOLUTE),
    (sbc, AddressMode::ABSOLUTE),
    (inc, AddressMode::ABSOLUTE),
    (nop, AddressMode::NoMode),
    (beq, AddressMode::NoMode),
    (sbc, AddressMode::INDIRECT_Y),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (sbc, AddressMode::ZERO_PAGE_X),
    (inc, AddressMode::ZERO_PAGE_X),
    (nop, AddressMode::NoMode),
    (sed, AddressMode::NoMode),
    (sbc, AddressMode::ABSOLUTE_X),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (nop, AddressMode::NoMode),
    (sbc, AddressMode::ABSOLUTE_Y),
    (inc, AddressMode::ABSOLUTE_X),
    (nop, AddressMode::NoMode),
];
