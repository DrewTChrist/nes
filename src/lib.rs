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

pub struct Nes<D>
where
    D: DrawTarget,
{
    pub apu: Apu,
    pub cpu: Cpu,
    pub ppu: Ppu<D>,
}

impl<D> Nes<D>
where
    D: DrawTarget,
{
    pub fn new(apu: Apu, cpu: Cpu, ppu: Ppu<D>) -> Self {
        Self { apu, cpu, ppu }
    }
}
