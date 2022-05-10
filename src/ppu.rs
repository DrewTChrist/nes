use embedded_graphics::{
    draw_target::DrawTarget, pixelcolor::Rgb565, prelude::*, 
};

/// Picture processing unit
pub struct Ppu<D>
where
    D: DrawTarget,
{
    display: D,
}

impl<D> Ppu<D>
where
    D: OriginDimensions + DrawTarget<Color = Rgb565>,
{
    pub fn new(display: D) -> Self
    where
        D: OriginDimensions + DrawTarget<Color = Rgb565>,
    {
        Self { display }
    }
}
