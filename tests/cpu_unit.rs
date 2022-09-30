#[cfg(test)]
mod cpu {
    use nes::cpu::Cpu;

    #[test]
    fn get_mem_slice() {
        let mut cpu = Cpu::new();
        cpu.write_mem(0x8000, 0x1);
        cpu.write_mem(0x8001, 0x2);
        cpu.write_mem(0x8002, 0x3);
        let slice = cpu.get_mem_slice(0x8000, 0x8003);
        assert_eq!(slice.len(), 3);
        assert_eq!(slice[2], 0x3);
    }
}

#[cfg(test)]
mod instructions {
    use nes::cpu::Cpu;

    #[test]
    fn _00() {
        // brk
        let program: [u8; 2] = [0x00, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0xfffd, 0x03);
        cpu.write_mem(0xfffe, 0x80);
        cpu.tick();
        assert_eq!(cpu.reg.p & 0b0010_0000, 0b0010_0000);
        assert_eq!(cpu.reg.pc, 0x8003);
    }

    #[test]
    fn _01() {
        // ora indirect x
        let program: [u8; 4] = [0x01, 0x64, 0x01, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x6e, 0x02);
        cpu.write_mem(0x6f, 0x80);
        cpu.reg.x = 0xa;
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0xb);
    }

    #[test]
    fn _05() {
        // ora zero page
        let program: [u8; 3] = [0x05, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x64, 0x1);
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0xb);
    }

    #[test]
    fn _06() {
        // asl zero page
        let program: [u8; 3] = [0x06, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x64, 0xa);
        cpu.tick();
        assert_eq!(cpu.read_mem(0x64), 0x14);
    }

    #[test]
    fn _08() {
        // php
        let program: [u8; 2] = [0x08, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p = 0xa;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x01ff), 0xa);
    }

    #[test]
    fn _09() {
        // ora immediate
        let program: [u8; 3] = [0x09, 0x1, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0xb);
    }

    #[test]
    fn _0a() {
        // asl accumulator
        let program: [u8; 2] = [0x0a, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0x14);
    }

    #[test]
    fn _0d() {
        // ora absolute
        let program: [u8; 5] = [0x0d, 0x03, 0x80, 0x01, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0xb);
    }

    #[test]
    fn _0e() {
        // asl absolute
        let program: [u8; 5] = [0x0e, 0x03, 0x80, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.read_mem(0x8003), 0x14);
    }

    #[test]
    fn _10_pos() {
        // bpl with a positive relative offset
        let program: [u8; 3] = [0x10, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8066);
    }

    #[test]
    fn _10_neg() {
        // bpl with a negative relative offset
        let program: [u8; 3] = [0x10, 0x9c, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x7f9e);
    }

    #[test]
    fn _11() {
        // ora indirect y
        let program: [u8; 4] = [0x11, 0x64, 0x01, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x64, 0x00);
        cpu.write_mem(0x65, 0x80);
        cpu.reg.y = 0x2;
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0xb);
    }

    #[test]
    fn _15() {
        // ora zero page x
        let program: [u8; 3] = [0x15, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x6e, 0x1);
        cpu.reg.x = 0xa;
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0xb);
    }

    #[test]
    fn _16() {
        // asl zero page x
        let program: [u8; 3] = [0x16, 0x60, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x4;
        cpu.write_mem(0x64, 0xa);
        cpu.tick();
        assert_eq!(cpu.read_mem(0x64), 0x14);
    }

    #[test]
    fn _18() {
        // clc
        let program: [u8; 1] = [0x18];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p = 0b0000_0001;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8001);
        assert!(cpu.reg.p & 0b0000_0001 == 0b00);
    }

    #[test]
    fn _19() {
        // ora absolute y
        let program: [u8; 5] = [0x19, 0x00, 0x80, 0x01, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x03;
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0xb);
    }

    #[test]
    fn _1d() {
        // ora absolute x
        let program: [u8; 5] = [0x1d, 0x00, 0x80, 0x01, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x03;
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0xb);
    }

    #[test]
    fn _1e() {
        // asl absolute x
        let program: [u8; 5] = [0x1e, 0x00, 0x80, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x3;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x8003), 0x14);
    }

    #[test]
    fn _20() {
        // jsr
        let program: [u8; 4] = [0x20, 0x50, 0x80, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8050);
    }

    #[test]
    fn _21() {
        // and indirect x
        let program: [u8; 4] = [0x21, 0x05, 0x08, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0xa, 0x02);
        cpu.write_mem(0xb, 0x80);
        cpu.reg.x = 0x05;
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.a, 0x08);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _24() {
        // bit zero page
        let program: [u8; 3] = [0x24, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x64, 0xff);
        cpu.reg.a = 0xff;
        cpu.tick();
        assert_eq!(cpu.reg.p & 0x80, 0x80);
        assert_eq!(cpu.reg.p & 0x40, 0x40);
        assert_eq!(cpu.reg.p & 0x2, 0x0);
    }

    #[test]
    fn _24_zero() {
        // bit zero page but result is zero
        let program: [u8; 3] = [0x24, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x64, 0x00);
        cpu.reg.a = 0x42;
        cpu.tick();
        assert_eq!(cpu.reg.p & 0x80, 0x0);
        assert_eq!(cpu.reg.p & 0x40, 0x0);
        assert_eq!(cpu.reg.p & 0x2, 0x2);
    }

    #[test]
    fn _25() {
        // and zero page
        let program: [u8; 3] = [0x25, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x05, 0x08);
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.a, 0x08);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _26() {
        // rol zero page
        let program: [u8; 3] = [0x26, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x64, 0xa);
        cpu.reg.p |= 0x1;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x64), 0x15);
    }

    #[test]
    fn _28() {
        // pla
        let program: [u8; 2] = [0x28, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.push_stack(0xa);
        cpu.tick();
        assert_eq!(cpu.reg.p, 0xa);
    }

    #[test]
    fn _29() {
        // and immediate
        let program: [u8; 3] = [0x29, 0x08, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.a, 0x08);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _2a() {
        // rol accumulator
        let program: [u8; 2] = [0x2a, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0xa;
        cpu.reg.p |= 0x1;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0x15);
    }

    #[test]
    fn _2c() {
        // bit absolute
        let program: [u8; 5] = [0x2c, 0x03, 0x80, 0xff, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0xff;
        cpu.tick();
        assert_eq!(cpu.reg.p & 0x80, 0x80);
        assert_eq!(cpu.reg.p & 0x40, 0x40);
        assert_eq!(cpu.reg.p & 0x2, 0x0);
    }

    #[test]
    fn _2c_zero() {
        // bit absolute but result is zero
        let program: [u8; 5] = [0x2c, 0x03, 0x80, 0x00, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0x42;
        cpu.tick();
        assert_eq!(cpu.reg.p & 0x80, 0x0);
        assert_eq!(cpu.reg.p & 0x40, 0x0);
        assert_eq!(cpu.reg.p & 0x2, 0x2);
    }

    #[test]
    fn _2d() {
        // and absolute
        let program: [u8; 5] = [0x2d, 0x03, 0x80, 0x08, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8003);
        assert_eq!(cpu.reg.a, 0x08);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _2e() {
        // rol absolute
        let program: [u8; 5] = [0x2e, 0x03, 0x80, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p |= 0x1;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x8003), 0x15);
    }

    #[test]
    fn _30_pos() {
        // bmi with a positive relative offset
        let program: [u8; 3] = [0x30, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p |= 0b1000_0000;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8066);
    }

    #[test]
    fn _30_neg() {
        // bmi with a negative relative offset
        let program: [u8; 3] = [0x30, 0x9c, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p |= 0b1000_0000;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x7f9e);
    }

    #[test]
    fn _31() {
        // and indirect y
        let program: [u8; 4] = [0x31, 0x05, 0x08, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x05, 0x00);
        cpu.write_mem(0x06, 0x80);
        cpu.reg.y = 0x02;
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.a, 0x08);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _35() {
        // and zero page x
        let program: [u8; 3] = [0x35, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x05;
        cpu.write_mem(0xa, 0x08);
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.a, 0x08);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _36() {
        // rol zero page x
        let program: [u8; 3] = [0x36, 0x60, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x64, 0xa);
        cpu.reg.p |= 0x1;
        cpu.reg.x = 0x4;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x64), 0x15);
    }

    #[test]
    fn _38() {
        // sec
        let program: [u8; 1] = [0x38];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p = 0b0000_0000;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8001);
        assert!(cpu.reg.p & 0b0000_0001 != 0b00);
    }

    #[test]
    fn _39() {
        // and absolute y
        let program: [u8; 5] = [0x39, 0x00, 0x80, 0x08, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x03;
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8003);
        assert_eq!(cpu.reg.a, 0x08);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _3d() {
        // and absolute x
        let program: [u8; 5] = [0x3d, 0x00, 0x80, 0x08, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x03;
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8003);
        assert_eq!(cpu.reg.a, 0x08);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _3e() {
        // rol absolute x
        let program: [u8; 5] = [0x3e, 0x00, 0x80, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p |= 0x1;
        cpu.reg.x = 0x3;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x8003), 0x15);
    }

    #[test]
    fn _40() {
        // rti
        let program: [u8; 2] = [0x00, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0xfffd, 0x03);
        cpu.write_mem(0xfffe, 0x80);
        cpu.write_mem(0x8003, 0x40);
        cpu.reg.p = 0x45;
        cpu.tick();
        cpu.tick();
        assert_eq!(cpu.reg.p, 0x45);
        assert_eq!(cpu.reg.pc, 0x8001);
    }

    #[test]
    fn _41() {
        // eor indirect x
        let program: [u8; 4] = [0x41, 0x05, 0xb, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x05;
        cpu.reg.a = 0xa;
        cpu.write_mem(0xa, 0x02);
        cpu.write_mem(0xb, 0x80);
        cpu.tick();
        assert_eq!(cpu.reg.a, 0x1);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _46() {
        // lsr zero page
        let program: [u8; 3] = [0x46, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x64, 0xa);
        cpu.tick();
        assert_eq!(cpu.read_mem(0x64), 0x5);
    }

    #[test]
    fn _45() {
        // eor zero page
        let program: [u8; 3] = [0x45, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x05, 0xb);
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0x1);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _48() {
        // pha
        let program: [u8; 2] = [0x48, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x01ff), 0xa);
    }

    #[test]
    fn _49() {
        // eor immediate
        let program: [u8; 3] = [0x49, 0xb, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0x1);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _4a() {
        // lsr accumulator
        let program: [u8; 2] = [0x4a, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0x5);
    }

    #[test]
    fn _4c() {
        // jmp absolute
        let program: [u8; 4] = [0x4c, 0x00, 0x90, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x9000);
    }

    #[test]
    fn _4d() {
        // eor absolute
        let program: [u8; 5] = [0x4d, 0x03, 0x80, 0xb, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0x1);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _4e() {
        // lsr absolute
        let program: [u8; 5] = [0x4e, 0x03, 0x80, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.read_mem(0x8003), 0x5);
    }

    #[test]
    fn _50_pos() {
        // bvc with a positive relative offset
        let program: [u8; 3] = [0x10, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8066);
    }

    #[test]
    fn _50_neg() {
        // bvc with a negative relative offset
        let program: [u8; 3] = [0x10, 0x9c, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x7f9e);
    }

    #[test]
    fn _51() {
        // eor indirect y
        let program: [u8; 4] = [0x51, 0xa, 0xb, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x02;
        cpu.reg.a = 0xa;
        cpu.write_mem(0xa, 0x00);
        cpu.write_mem(0xb, 0x80);
        cpu.tick();
        assert_eq!(cpu.reg.a, 0x1);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _55() {
        // eor zero page x
        let program: [u8; 3] = [0x55, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0xa, 0xb);
        cpu.reg.x = 0x05;
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0x1);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _56() {
        // lsr zero page x
        let program: [u8; 3] = [0x56, 0x60, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x4;
        cpu.write_mem(0x64, 0xa);
        cpu.tick();
        assert_eq!(cpu.read_mem(0x64), 0x5);
    }

    #[test]
    fn _58() {
        // cli
        let program: [u8; 1] = [0x58];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p = 0b0000_0100;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8001);
        assert!(cpu.reg.p & 0b0000_0100 == 0b00);
    }

    #[test]
    fn _59() {
        // eor absolute y
        let program: [u8; 5] = [0x59, 0x00, 0x80, 0xb, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x03;
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0x1);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _5d() {
        // eor absolute x
        let program: [u8; 5] = [0x5d, 0x00, 0x80, 0xb, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x03;
        cpu.reg.a = 0xa;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0x1);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _5e() {
        // lsr absolute x
        let program: [u8; 5] = [0x5e, 0x00, 0x80, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x3;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x8003), 0x5);
    }

    #[test]
    fn _60() {
        // rts
        let program: [u8; 4] = [0x20, 0x50, 0x80, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x8050, 0x60);
        cpu.tick();
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8003);
    }

    #[test]
    fn _60_nested() {
        // rts with two nested "subroutines"
        let program: [u8; 4] = [0x20, 0x50, 0x80, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x8050, 0x20);
        cpu.write_mem(0x8051, 0x50);
        cpu.write_mem(0x8052, 0x90);
        cpu.write_mem(0x8053, 0x60);
        cpu.write_mem(0x9050, 0x60);
        cpu.tick();
        cpu.tick();
        cpu.tick();
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8003);
    }

    #[test]
    fn _61() {
        // adc indirect x
    }

    #[test]
    fn _65() {
        // adc zero page
    }

    #[test]
    fn _66() {
        // ror zero page
        let program: [u8; 3] = [0x66, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x64, 0xa);
        cpu.reg.p |= 0x1;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x64), 0x85);
    }

    #[test]
    fn _68() {
        // pla
        let program: [u8; 2] = [0x68, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.push_stack(0xa);
        cpu.tick();
        assert_eq!(cpu.reg.a, 0xa);
    }

    #[test]
    fn _69() {
        // adc immediate
    }

    #[test]
    fn _6a() {
        // ror accumulator
        let program: [u8; 2] = [0x6a, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0xa;
        cpu.reg.p |= 0x1;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0x85);
    }

    #[test]
    fn _6c() {
        // jmp indirect
        let program: [u8; 4] = [0x6c, 0x00, 0x90, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x9000);
    }

    #[test]
    fn _6c_page_bound() {
        // jmp indirect on a page boundary
        let mut cpu = Cpu::new();
        cpu.write_mem(0x81fe, 0x6c);
        cpu.write_mem(0x81ff, 0x00);
        cpu.write_mem(0x8100, 0x90);
        cpu.reg.pc = 0x81fe;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x9000);
    }

    #[test]
    fn _6d() {
        // adc absolute
    }

    #[test]
    fn _6e() {
        // ror absolute
        let program: [u8; 5] = [0x6e, 0x03, 0x80, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p |= 0x1;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x8003), 0x85);
    }

    #[test]
    fn _70_pos() {
        // bvs with a positive relative offset
        let program: [u8; 3] = [0x70, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p |= 0b0100_0000;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8066);
    }

    #[test]
    fn _70_neg() {
        // bvs with a negative relative offset
        let program: [u8; 3] = [0x70, 0x9c, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p |= 0b0100_0000;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x7f9e);
    }

    #[test]
    fn _71() {
        // adc indirect y
    }

    #[test]
    fn _75() {
        // adc zero page x
    }

    #[test]
    fn _76() {
        // rol zero page x
        let program: [u8; 3] = [0x76, 0x60, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x64, 0xa);
        cpu.reg.p |= 0x1;
        cpu.reg.x = 0x4;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x64), 0x85);
    }

    #[test]
    fn _78() {
        // sei
        let program: [u8; 1] = [0x78];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p = 0b0000_0000;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8001);
        assert!(cpu.reg.p & 0b0000_0100 != 0b00);
    }

    #[test]
    fn _79() {
        // adc absolute y
    }

    #[test]
    fn _7d() {
        // adc absolute x
    }

    #[test]
    fn _7e() {
        // ror absolute x
        let program: [u8; 5] = [0x7e, 0x00, 0x80, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p |= 0x1;
        cpu.reg.x = 0x3;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x8003), 0x85);
    }

    #[test]
    fn _81() {
        // sta indirect x
        let program: [u8; 3] = [0x81, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x05;
        cpu.reg.a = 0x32;
        cpu.write_mem(0xa, 0x02);
        cpu.write_mem(0xb, 0x80);
        cpu.tick();
        assert_eq!(cpu.read_mem(0x8002), 0x32);
    }

    #[test]
    fn _84() {
        // sty zero page
        let program: [u8; 3] = [0x84, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x32;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x64), 0x32);
    }

    #[test]
    fn _85() {
        // sta zero page
        let program: [u8; 3] = [0x85, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0x32;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x64), 0x32);
    }

    #[test]
    fn _86() {
        // stx zero page
        let program: [u8; 3] = [0x86, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x32;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x64), 0x32);
    }

    #[test]
    fn _88() {
        // dey
        let program: [u8; 3] = [0x88, 0x88, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x05;
        cpu.tick();
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.y, 3);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _8a() {
        // txa
        let program: [u8; 2] = [0x8a, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x10;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0x10);
        assert_eq!(cpu.reg.pc, 0x8001);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _8c() {
        // sty absolute
        let program: [u8; 4] = [0x8c, 0x03, 0x80, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x32;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x8003), 0x32);
    }

    #[test]
    fn _8d() {
        // sta absolute
        let program: [u8; 4] = [0x8d, 0x03, 0x80, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0x32;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x8003), 0x32);
    }

    #[test]
    fn _8e() {
        // stx absolute
        let program: [u8; 4] = [0x8e, 0x03, 0x80, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x32;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x8003), 0x32);
    }

    #[test]
    fn _90_pos() {
        // bcc with a positive relative offset
        let program: [u8; 3] = [0x90, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8066);
    }

    #[test]
    fn _90_neg() {
        // bcc with a negative relative offset
        let program: [u8; 3] = [0x90, 0x9c, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x7f9e);
    }

    #[test]
    fn _91() {
        // sta indirect y
        let program: [u8; 3] = [0x91, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x01;
        cpu.reg.a = 0x32;
        cpu.write_mem(0xa, 0x01);
        cpu.write_mem(0xb, 0x80);
        cpu.tick();
        assert_eq!(cpu.read_mem(0x8001), 0x32);
    }

    #[test]
    fn _94() {
        // sty zero page x
        let program: [u8; 3] = [0x94, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x2;
        cpu.reg.y = 0x32;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x66), 0x32);
    }

    #[test]
    fn _95() {
        // sta zero page x
        let program: [u8; 3] = [0x95, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x2;
        cpu.reg.a = 0x32;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x66), 0x32);
    }

    #[test]
    fn _96() {
        // stx zero page y
        let program: [u8; 3] = [0x96, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x2;
        cpu.reg.x = 0x32;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x66), 0x32);
    }

    #[test]
    fn _98() {
        // tya
        let program: [u8; 2] = [0x98, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x10;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0x10);
        assert_eq!(cpu.reg.pc, 0x8001);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _99() {
        // sta absolute y
        let program: [u8; 4] = [0x99, 0x03, 0x80, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x1;
        cpu.reg.a = 0x32;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x8004), 0x32);
    }

    #[test]
    fn _9a() {
        // txs
        let program: [u8; 2] = [0x9a, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x10;
        cpu.tick();
        assert_eq!(cpu.reg.s, 0x10);
        assert_eq!(cpu.reg.pc, 0x8001);
    }

    #[test]
    fn _9d() {
        // sta absolute x
        let program: [u8; 4] = [0x9d, 0x03, 0x80, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x1;
        cpu.reg.a = 0x32;
        cpu.tick();
        assert_eq!(cpu.read_mem(0x8004), 0x32);
    }

    #[test]
    fn _a0() {
        // ldy immediate
        let program: [u8; 3] = [0xa0, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.y, 0xa);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _a1() {
        // lda indirect x
        let program: [u8; 4] = [0xa1, 0x05, 0xb, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x05;
        cpu.write_mem(0xa, 0x02);
        cpu.write_mem(0xb, 0x80);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.a, 0xb);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _a2() {
        // ldx immediate
        let program: [u8; 3] = [0xa2, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.x, 0xa);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _a4() {
        // ldy zero page
        let program: [u8; 3] = [0xa4, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x05, 0xa);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.y, 0xa);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _a5() {
        // lda zero page
        let program: [u8; 3] = [0xa5, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.write_mem(0x05, 0x25);
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.a, 0x25);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _a6() {
        // ldx zero page
        let program: [u8; 3] = [0xa6, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x05, 0xa);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.x, 0xa);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _a8() {
        // tay
        let program: [u8; 2] = [0xa8, 0x00];
        let mut cpu = Cpu::new();
        cpu.reg.a = 10;
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.y, 10);
        assert_eq!(cpu.reg.pc, 0x8001);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _a9() {
        // lda immediate
        let program: [u8; 3] = [0xa9, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.a, 0x05);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _aa() {
        // tax
        let program: [u8; 2] = [0xaa, 0x00];
        let mut cpu = Cpu::new();
        cpu.reg.a = 10;
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.x, 10);
        assert_eq!(cpu.reg.pc, 0x8001);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _ac() {
        // ldy absolute
        let program: [u8; 5] = [0xac, 0x03, 0x80, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8003);
        assert_eq!(cpu.reg.y, 0xa);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _ad() {
        // lda absolute
        let program: [u8; 5] = [0xad, 0x03, 0x80, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.a, 0xa);
        assert_eq!(cpu.reg.pc, 0x8003);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _ae() {
        // ldx absolute
        let program: [u8; 5] = [0xae, 0x03, 0x80, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8003);
        assert_eq!(cpu.reg.x, 0xa);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _b0_pos() {
        // bcs with a positive relative offset
        let program: [u8; 3] = [0xb0, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p |= 0b0000_0001;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8066);
    }

    #[test]
    fn _b0_neg() {
        // bcs with a negative relative offset
        let program: [u8; 3] = [0xb0, 0x9c, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p |= 0b0000_0001;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x7f9e);
    }

    #[test]
    fn _b1() {
        // lda indirect y
        let program: [u8; 4] = [0xb1, 0x05, 0xb, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x02;
        cpu.write_mem(0x05, 0x00);
        cpu.write_mem(0x06, 0x80);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.a, 0xb);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _b4() {
        // ldy zero page x
        let program: [u8; 3] = [0xb4, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x02;
        cpu.write_mem(0x07, 0xa);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.y, 0xa);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _b5() {
        // lda zero page x
        let program: [u8; 3] = [0xb5, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.reg.x = 0x05;
        cpu.write_mem(0xa, 0x25);
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.a, 0x25);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _b6() {
        // ldx zero page x
        let program: [u8; 3] = [0xb6, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x02;
        cpu.write_mem(0x07, 0xa);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.x, 0xa);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _b8() {
        // clv
        let program: [u8; 1] = [0xb8];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p = 0b0000_0000;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8001);
        assert!(cpu.reg.p & 0b0100_0000 == 0b00);
    }

    #[test]
    fn _b9() {
        // lda absolute x
        let program: [u8; 5] = [0xb9, 0x00, 0x80, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x03;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0xa);
        assert_eq!(cpu.reg.pc, 0x8003);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _ba() {
        // tsx
        let program: [u8; 2] = [0xba, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.s = 0x10;
        cpu.tick();
        assert_eq!(cpu.reg.x, 0x10);
        assert_eq!(cpu.reg.pc, 0x8001);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _bc() {
        // ldy absolute x
        let program: [u8; 5] = [0xbc, 0x00, 0x80, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x03;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8003);
        assert_eq!(cpu.reg.y, 0xa);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _bd() {
        // lda absolute x
        let program: [u8; 5] = [0xbd, 0x00, 0x80, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x03;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0xa);
        assert_eq!(cpu.reg.pc, 0x8003);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _be() {
        // ldx absolute x
        let program: [u8; 5] = [0xbe, 0x00, 0x80, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x03;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8003);
        assert_eq!(cpu.reg.x, 0xa);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _c0_pos() {
        // cpy immediate
        // Y >= M
        let program: [u8; 3] = [0xc0, 0x19, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x32;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _c0_eq() {
        // cpy immediate
        // Y = M
        let program: [u8; 3] = [0xc0, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x32;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 != 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _c0_neg() {
        // cpy immediate
        // Y < M
        let program: [u8; 3] = [0xc0, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x19;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 == 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 != 0);
    }

    #[test]
    fn _c1_pos() {
        // cmp indirect x
        // A >= M
        let program: [u8; 4] = [0xc1, 0x64, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x64;
        cpu.reg.a = 0x64;
        cpu.write_mem(0xc8, 0x02);
        cpu.write_mem(0xc9, 0x80);
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _c1_eq() {
        // cmp indirect x
        // A = M
        let program: [u8; 4] = [0xc1, 0x64, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x64;
        cpu.reg.a = 0x32;
        cpu.write_mem(0xc8, 0x02);
        cpu.write_mem(0xc9, 0x80);
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 != 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _c1_neg() {
        // cmp indirect x
        // A < M
        let program: [u8; 4] = [0xc1, 0x64, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x64;
        cpu.reg.a = 0x32;
        cpu.write_mem(0xc8, 0x02);
        cpu.write_mem(0xc9, 0x80);
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 == 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 != 0);
    }

    #[test]
    fn _c4_pos() {
        // cpy zero page
        // Y >= M
        let program: [u8; 3] = [0xc4, 0xc8, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x32;
        cpu.write_mem(0xc8, 0x19);
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _c4_eq() {
        // cpx zero page
        // Y = M
        let program: [u8; 3] = [0xc4, 0xc8, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x32;
        cpu.write_mem(0xc8, 0x32);
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 != 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _c4_neg() {
        // cpx zero page
        // Y < M
        let program: [u8; 3] = [0xc4, 0xc8, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x19;
        cpu.write_mem(0xc8, 0x32);
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 == 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 != 0);
    }

    #[test]
    fn _c5_pos() {
        // cmp zero page
        // A >= M
        let program: [u8; 3] = [0xc5, 0xc8, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0x32;
        cpu.write_mem(0xc8, 0x19);
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _c5_eq() {
        // cmp zero page
        // A = M
        let program: [u8; 3] = [0xc5, 0xc8, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0x32;
        cpu.write_mem(0xc8, 0x32);
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 != 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _c5_neg() {
        // cmp zero page
        // A < M
        let program: [u8; 3] = [0xc5, 0xc8, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0x19;
        cpu.write_mem(0xc8, 0x32);
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 == 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 != 0);
    }

    #[test]
    fn _c6() {
        // dec zero page
        let program: [u8; 3] = [0xc6, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x05, 0xa);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.read_mem(0x05), 0x09);
    }

    #[test]
    fn _c8() {
        // iny
        let program: [u8; 3] = [0xc8, 0xc8, 0x00];
        let mut cpu = Cpu::new();
        cpu.reg.y = 0xff;
        cpu.load_program(program);
        cpu.tick();
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.y, 1);
    }

    #[test]
    fn _c9_pos() {
        // cmp immediate
        // A >= M
        let program: [u8; 3] = [0xc9, 0x19, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0x32;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _c9_eq() {
        // cmp immediate
        // A = M
        let program: [u8; 3] = [0xc9, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0x32;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 != 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _c9_neg() {
        // cmp immediate
        // A < M
        let program: [u8; 3] = [0xc9, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0x19;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 == 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 != 0);
    }

    #[test]
    fn _ca() {
        // dex
        let program: [u8; 3] = [0xca, 0xca, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x05;
        cpu.tick();
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.x, 3);
        assert!(cpu.reg.p & 0b0000_0010 == 0b00);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _cc_pos() {
        // cpx absolute
        // Y >= M
        let program: [u8; 5] = [0xcc, 0x03, 0x80, 0x19, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x32;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _cc_eq() {
        // cpx absolute
        // Y = M
        let program: [u8; 5] = [0xcc, 0x03, 0x80, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x32;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 != 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _cc_neg() {
        // cpx absolute
        // Y < M
        let program: [u8; 5] = [0xcc, 0x03, 0x80, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x19;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 == 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 != 0);
    }

    #[test]
    fn _cd_pos() {
        // cmp absolute
        // A >= M
        let program: [u8; 5] = [0xcd, 0x03, 0x80, 0x19, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0x32;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _cd_eq() {
        // cmp absolute
        // A = M
        let program: [u8; 5] = [0xcd, 0x03, 0x80, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0x32;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 != 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _cd_neg() {
        // cmp absolute
        // A < M
        let program: [u8; 5] = [0xcd, 0x03, 0x80, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.a = 0x19;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 == 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 != 0);
    }

    #[test]
    fn _ce() {
        // dec absolute
        let program: [u8; 5] = [0xce, 0x03, 0x80, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8003);
        assert_eq!(cpu.read_mem(0x8003), 0x09);
    }

    #[test]
    fn _d0_pos() {
        // bne with a positive relative offset
        let program: [u8; 3] = [0xd0, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8066);
    }

    #[test]
    fn _d0_neg() {
        // bne with a negative relative offset
        let program: [u8; 3] = [0xd0, 0x9c, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x7f9e);
    }

    #[test]
    fn _d1_pos() {
        // cmp indirect y
        // A >= M
        let program: [u8; 4] = [0xd1, 0x64, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x02;
        cpu.reg.a = 0x64;
        cpu.write_mem(0x64, 0x00);
        cpu.write_mem(0x65, 0x80);
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _d1_eq() {
        // cmp indirect y
        // A = M
        let program: [u8; 4] = [0xd1, 0x64, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x02;
        cpu.reg.a = 0x32;
        cpu.write_mem(0x64, 0x00);
        cpu.write_mem(0x65, 0x80);
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 != 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _d1_neg() {
        // cmp indirect y
        // A < M
        let program: [u8; 4] = [0xd1, 0x64, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x02;
        cpu.reg.a = 0x19;
        cpu.write_mem(0x64, 0x00);
        cpu.write_mem(0x65, 0x80);
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 == 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 != 0);
    }

    #[test]
    fn _d5_pos() {
        // cmp zero page x
        // A >= M
        let program: [u8; 3] = [0xd5, 0xc8, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0xa;
        cpu.reg.a = 0x32;
        cpu.write_mem(0xd2, 0x19);
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _d5_eq() {
        // cmp zero page x
        // A = M
        let program: [u8; 3] = [0xd5, 0xc8, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0xa;
        cpu.reg.a = 0x32;
        cpu.write_mem(0xd2, 0x32);
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 != 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _d5_neg() {
        // cmp zero page x
        // A < M
        let program: [u8; 3] = [0xd5, 0xc8, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0xa;
        cpu.reg.a = 0x19;
        cpu.write_mem(0xd2, 0x32);
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 == 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 != 0);
    }

    #[test]
    fn _d6() {
        // dec zero page x
        let program: [u8; 3] = [0xd6, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x05;
        cpu.write_mem(0xa, 0xa);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.read_mem(0xa), 0x09);
    }

    #[test]
    fn _d8() {
        // cld
        let program: [u8; 1] = [0xd8];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p = 0b0000_1000;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8001);
        assert!(cpu.reg.p & 0b0000_1000 == 0b00);
    }

    #[test]
    fn _d9_pos() {
        // cmp absolute y
        // A >= M
        let program: [u8; 5] = [0xd9, 0x01, 0x80, 0x19, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x02;
        cpu.reg.a = 0x32;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _d9_eq() {
        // cmp absolute y
        // A = M
        let program: [u8; 5] = [0xd9, 0x01, 0x80, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x02;
        cpu.reg.a = 0x32;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 != 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _d9_neg() {
        // cmp absolute y
        // A < M
        let program: [u8; 5] = [0xd9, 0x01, 0x80, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.y = 0x02;
        cpu.reg.a = 0x19;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 == 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 != 0);
    }

    #[test]
    fn _dd_pos() {
        // cmp absolute x
        // A >= M
        let program: [u8; 5] = [0xdd, 0x01, 0x80, 0x19, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x02;
        cpu.reg.a = 0x32;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _dd_eq() {
        // cmp absolute x
        // A = M
        let program: [u8; 5] = [0xdd, 0x01, 0x80, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x02;
        cpu.reg.a = 0x32;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 != 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _dd_neg() {
        // cmp absolute x
        // A < M
        let program: [u8; 5] = [0xdd, 0x01, 0x80, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x02;
        cpu.reg.a = 0x19;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 == 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 != 0);
    }

    #[test]
    fn _de() {
        // dec absolute x
        let program: [u8; 5] = [0xde, 0x00, 0x80, 0xa, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x03;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8003);
        assert_eq!(cpu.read_mem(0x8003), 0x09);
    }

    #[test]
    fn _e0_pos() {
        // cpx immediate
        // X >= M
        let program: [u8; 3] = [0xe0, 0x19, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x32;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _e0_eq() {
        // cpx immediate
        // X = M
        let program: [u8; 3] = [0xe0, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x32;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 != 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _e0_neg() {
        // cpx immediate
        // X < M
        let program: [u8; 3] = [0xe0, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x19;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 == 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 != 0);
    }

    #[test]
    fn _e4_pos() {
        // cpx zero page
        // X >= M
        let program: [u8; 3] = [0xe4, 0xc8, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x32;
        cpu.write_mem(0xc8, 0x19);
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _e4_eq() {
        // cpx zero page
        // X = M
        let program: [u8; 3] = [0xe4, 0xc8, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x32;
        cpu.write_mem(0xc8, 0x32);
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 != 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _e4_neg() {
        // cpx zero page
        // X < M
        let program: [u8; 3] = [0xe4, 0xc8, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x19;
        cpu.write_mem(0xc8, 0x32);
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 == 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 != 0);
    }

    #[test]
    fn _e6() {
        // inc zero page
        let program: [u8; 3] = [0xe6, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x05, 0x09);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.read_mem(0x05), 0xa);
    }

    #[test]
    fn _e8() {
        // inx
        let program: [u8; 3] = [0xe8, 0xe8, 0x00];
        let mut cpu = Cpu::new();
        cpu.reg.x = 0xff;
        cpu.load_program(program);
        cpu.tick();
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.x, 1);
    }

    #[test]
    fn _ec_pos() {
        // cpx absolute
        // X >= M
        let program: [u8; 5] = [0xec, 0x03, 0x80, 0x19, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x32;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _ec_eq() {
        // cpx absolute
        // X = M
        let program: [u8; 5] = [0xec, 0x03, 0x80, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x32;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 != 0);
        assert!(cpu.reg.p & 0b0000_0010 != 0);
        assert!(cpu.reg.p & 0b1000_0000 == 0);
    }

    #[test]
    fn _ec_neg() {
        // cpx absolute
        // X < M
        let program: [u8; 5] = [0xec, 0x03, 0x80, 0x32, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x19;
        cpu.tick();
        assert!(cpu.reg.p & 0b0000_0001 == 0);
        assert!(cpu.reg.p & 0b0000_0010 == 0);
        assert!(cpu.reg.p & 0b1000_0000 != 0);
    }

    #[test]
    fn _ee() {
        // inc absolute
        let program: [u8; 5] = [0xee, 0x03, 0x80, 0x09, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8003);
        assert_eq!(cpu.read_mem(0x8003), 0xa);
    }

    #[test]
    fn _f0_pos() {
        // beq with a positive relative offset
        let program: [u8; 3] = [0xf0, 0x64, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p |= 0b0000_0010;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8066);
    }

    #[test]
    fn _f0_neg() {
        // beq with a negative relative offset
        let program: [u8; 3] = [0xf0, 0x9c, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p |= 0b0000_0010;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x7f9e);
    }

    #[test]
    fn _f6() {
        // inc zero page x
        let program: [u8; 3] = [0xf6, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.x = 0x02;
        cpu.write_mem(0x07, 0x09);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.read_mem(0x07), 0xa);
    }

    #[test]
    fn _f8() {
        // sed
        let program: [u8; 1] = [0xf8];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.reg.p = 0b0000_0000;
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8001);
        assert!(cpu.reg.p & 0b0000_1000 != 0b00);
    }

    #[test]
    fn _fe() {
        // inc absolute x
        let program: [u8; 5] = [0xfe, 0x00, 0x80, 0x09, 0x00];
        let mut cpu = Cpu::new();
        cpu.reg.x = 0x03;
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8003);
        assert_eq!(cpu.read_mem(0x8003), 0xa);
    }

    #[test]
    fn five_ops() {
        // Just tests five consecutive opcodes
        let program: [u8; 5] = [0xa9, 0xc0, 0xaa, 0xe8, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        cpu.tick();
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8004);
        assert_eq!(cpu.reg.x, 0xc1);
    }

    #[test]
    fn dec_test() {
        // Tests repeated decrements
        let program: [u8; 19] = [
            0xce, 0x05, 0x00, 0xce, 0x05, 0x00, 0xce, 0x05, 0x00, 0xce, 0x05, 0x00, 0xce, 0x05,
            0x00, 0xce, 0x05, 0x00, 0x00,
        ];

        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.write_mem(0x05, 0xff);
        for _ in 0..6 {
            cpu.tick();
        }
        assert_eq!(cpu.reg.pc, 0x8012);
        assert_eq!(cpu.read_mem(0x05), 0xff - 0x06);
    }
}
