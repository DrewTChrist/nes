#[cfg(test)]
mod apu {}

#[cfg(test)]
mod cpu {
    use nes::cpu::Cpu;

    #[test]
    fn a9() {
        let program: [u8; 3] = [0xa9, 0x05, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        for _ in 0..program.len() {
            cpu.tick();
        }
        assert_eq!(cpu.reg.a, 0x05);
        assert!(cpu.reg.s & 0b0000_0010 == 0b00);
        assert!(cpu.reg.s & 0b1000_0000 == 0);
    }

    #[test]
    fn aa() {
        let program: [u8; 2] = [0xaa, 0x00];
        let mut cpu = Cpu::new();
        cpu.reg.a = 10;
        cpu.load_program(program);
        for _ in 0..program.len() {
            cpu.tick();
        }
        assert_eq!(cpu.reg.x, 10);
        assert!(cpu.reg.s & 0b0000_0010 == 0b00);
        assert!(cpu.reg.s & 0b1000_0000 == 0);
    }
    
    #[test]
    fn five_ops() {
        let program: [u8; 5] = [0xa9, 0xc0, 0xaa, 0xe8, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        for _ in 0..program.len() {
            cpu.tick();
        }
        assert_eq!(cpu.reg.x, 0xc1);
    }

    #[test]
    fn e8() {
        let program: [u8; 3] = [0xe8, 0xe8, 0x00];
        let mut cpu = Cpu::new();
        cpu.reg.x = 0xff;
        cpu.load_program(program);
        for _ in 0..program.len() {
            cpu.tick();
        }
        assert_eq!(cpu.reg.x, 1);
    }
}

#[cfg(test)]
mod ppu {
    use nes::ppu::Ppu;
    use embedded_graphics::mock_display::MockDisplay;
    use embedded_graphics::pixelcolor::Rgb565;

    #[test]
    fn mock_display() {
        let mock_display = MockDisplay::<Rgb565>::new();
        let _ppu = Ppu::new(mock_display);
    }
}
