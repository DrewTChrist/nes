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
