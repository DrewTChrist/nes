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

use embedded_graphics::prelude::DrawTarget;
use embedded_hal::serial::{Read, Write};

#[cfg(not(feature = "debug"))]
pub struct Nes<D>
where
    D: DrawTarget,
{
    pub apu: Apu,
    pub cpu: Cpu,
    pub ppu: Ppu<D>,
}

#[cfg(not(feature = "debug"))]
impl<D> Nes<D>
where
    D: DrawTarget,
{
    pub fn new(apu: Apu, cpu: Cpu, ppu: Ppu<D>) -> Self {
        Self { apu, cpu, ppu }
    }
}

#[cfg(feature = "debug")]
pub struct Nes<D, S>
where
    D: DrawTarget,
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
    D: DrawTarget,
    S: Read<u8> + Write<u8>,
{
    pub fn new(apu: Apu, cpu: Cpu, ppu: Ppu<D>, serial: S) -> Self {
        Self {
            apu,
            cpu,
            ppu,
            serial,
        }
    }
}
