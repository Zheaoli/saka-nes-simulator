mod data;
mod headers;
mod mapper;
mod mapper0;
mod mapper1;
mod mapper2;
mod mapper3;
mod mapper4;
mod pager;

use self::{
    data::Data, mapper::Mapper, mapper0::Mapper0, mapper1::Mapper1, mapper2::Mapper2,
    mapper3::Mapper3, mapper4::Mapper4,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    None,
    //TODO: 4 screen
}

pub struct Cartridge {
    mapper: Box<dyn Mapper>,
}

impl Cartridge {
    pub fn new(data: &[u8]) -> Self {
        let data = Data::new(data);
        let mapper: Box<dyn Mapper> = match data.header.mapper_number {
            0 => Box::new(Mapper0::new(data)),
            1 => Box::new(Mapper1::new(data)),
            2 => Box::new(Mapper2::new(data)),
            3 => Box::new(Mapper3::new(data)),
            4 => Box::new(Mapper4::new(data)),
            n => panic!("Mapper {} not implemented yet", n),
        };
        Cartridge { mapper: mapper }
    }

    pub fn signal_scanline(&mut self) {
        self.mapper.signal_scanline();
    }

    pub fn read_prg_byte(&self, address: u16) -> u8 {
        self.mapper.read_prg_byte(address)
    }

    pub fn write_prg_byte(&mut self, address: u16, value: u8) {
        self.mapper.write_prg_byte(address, value);
    }

    pub fn read_chr_byte(&self, address: u16) -> u8 {
        self.mapper.read_chr_byte(address)
    }

    pub fn write_chr_byte(&mut self, address: u16, value: u8) {
        self.mapper.write_chr_byte(address, value)
    }

    pub fn mirroring(&self) -> Mirroring {
        self.mapper.mirroring()
    }

    pub fn irq_flag(&self) -> bool {
        self.mapper.irq_flag()
    }
}
