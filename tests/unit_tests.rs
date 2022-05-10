#[cfg(test)]
mod apu {}

#[cfg(test)]
mod cpu {
    use nes::cpu::Cpu;

    #[test]
    fn a9() {
        let program: [u8; 3] = [0x9a, 0x03, 0x00];
        let mut cpu = Cpu::new();
        cpu.load_program(program);
        for _ in 0..program.len() {
            cpu.tick();
        }
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
