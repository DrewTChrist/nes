#[cfg(test)]
mod apu {}

#[cfg(test)]
mod cpu {
    use nes::cpu::Cpu;

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

#[cfg(test)]
mod ppu {
    use embedded_graphics::mock_display::MockDisplay;
    use embedded_graphics::pixelcolor::Rgb565;
    use nes::ppu::Ppu;

    #[test]
    fn mock_display() {
        let mock_display = MockDisplay::<Rgb565>::new();
        let _ppu = Ppu::new(mock_display);
    }
}
