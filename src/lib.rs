#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod apu;
pub mod cpu;
pub mod ppu;

use apu::Apu;
use cpu::Cpu;
use ppu::Ppu;

use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565, prelude::*};
use embedded_hal::serial::{Read, Write};

#[cfg(not(feature = "debug"))]
pub struct Nes<D>
where
    D: OriginDimensions + DrawTarget<Color = Rgb565>,
{
    pub apu: Apu,
    pub cpu: Cpu,
    pub ppu: Ppu<D>,
}

#[cfg(not(feature = "debug"))]
impl<D> Nes<D>
where
    D: OriginDimensions + DrawTarget<Color = Rgb565>,
{
    pub fn new(display: D) -> Self {
        Self {
            apu: Apu::new(),
            cpu: Cpu::new(),
            ppu: Ppu::new(display),
        }
    }
}

#[cfg(feature = "debug")]
pub struct Nes<D, S>
where
    D: OriginDimensions + DrawTarget<Color = Rgb565>,
    S: Read<u8> + Write<u8>,
{
    pub apu: Apu,
    pub cpu: Cpu,
    pub ppu: Ppu<D>,
    serial: S,
}

#[cfg(feature = "debug")]
impl<D, S> Nes<D, S>
where
    D: OriginDimensions + DrawTarget<Color = Rgb565>,
    S: Read<u8> + Write<u8>,
{
    pub fn new(display: D, serial: S) -> Self {
        Self {
            apu: Apu::new(),
            cpu: Cpu::new(),
            ppu: Ppu::new(display),
            serial,
        }
    }
}
