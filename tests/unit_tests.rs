#[cfg(test)]
mod apu {}

#[cfg(test)]
mod cpu {
    use nes::cpu::Cpu;

    #[test]
    fn _21() {
        todo!();
    }

    #[test]
    fn _25() {
        todo!();
    }

    #[test]
    fn _29() {
        let program: [u8; 3] = [0x29, 0x08, 0x00];
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xa;
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.a, 0x08);
        assert!(cpu.reg.s & 0b0000_0010 == 0b00);
        assert!(cpu.reg.s & 0b1000_0000 == 0);
    }

    #[test]
    fn _2d() {
        todo!();
    }

    #[test]
    fn _31() {
        todo!();
    }

    #[test]
    fn _35() {
        todo!();
    }

    #[test]
    fn _39() {
        todo!();
    }

    #[test]
    fn _3d() {
        todo!();
    }

    #[test]
    fn _a1() {
        todo!();
    }

    #[test]
    fn _a5() {
        let program: [u8; 3] = [0xa5, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.write_mem(0x05, 0x25);
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.a, 0x25);
        assert!(cpu.reg.s & 0b0000_0010 == 0b00);
        assert!(cpu.reg.s & 0b1000_0000 == 0);
    }

    #[test]
    fn _a9() {
        let program: [u8; 3] = [0xa9, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.a, 0x05);
        assert!(cpu.reg.s & 0b0000_0010 == 0b00);
        assert!(cpu.reg.s & 0b1000_0000 == 0);
    }

    #[test]
    fn _aa() {
        let program: [u8; 2] = [0xaa, 0x00];
        let mut cpu = Cpu::new();
        cpu.reg.a = 10;
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.x, 10);
        assert!(cpu.reg.s & 0b0000_0010 == 0b00);
        assert!(cpu.reg.s & 0b1000_0000 == 0);
    }

    #[test]
    fn _ad() {
        todo!();
    }

    #[test]
    fn _b1() {
        todo!();
    }

    #[test]
    fn _b5() {
        let program: [u8; 3] = [0xb5, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.reg.x = 0x05;
        cpu.write_mem(0xa, 0x25);
        cpu.load_program(program);
        cpu.tick();
        assert_eq!(cpu.reg.pc, 0x8002);
        assert_eq!(cpu.reg.a, 0x25);
        assert!(cpu.reg.s & 0b0000_0010 == 0b00);
        assert!(cpu.reg.s & 0b1000_0000 == 0);
    }

    #[test]
    fn _b9() {
        todo!();
    }

    #[test]
    fn _bd() {
        todo!();
    }

    #[test]
    fn _e8() {
        let program: [u8; 3] = [0xe8, 0xe8, 0x00];
        let mut cpu = Cpu::new();
        cpu.reg.x = 0xff;
        cpu.load_program(program);
        cpu.tick();
        cpu.tick();
        assert_eq!(cpu.reg.x, 1);
    }

    #[test]
    fn five_ops() {
        let program: [u8; 5] = [0xa9, 0xc0, 0xaa, 0xe8, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        cpu.tick();
        cpu.tick();
        cpu.tick();
        assert_eq!(cpu.reg.x, 0xc1);
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
