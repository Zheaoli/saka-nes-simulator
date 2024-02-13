// Mapper3 implements ines mapper 3 (CNROM)
// https://wiki.nesdev.com/w/index.php/INES_Mapper_003

use super::pager::Page;
use super::pager::PageSize;
use super::Data;
use super::Mapper;
use super::Mirroring;

pub struct Mapper3 {
    data: Data,
    chr_0: usize,
}

impl Mapper3 {
    pub fn new(data: Data) -> Self {
        Mapper3 {
            data: data,
            chr_0: 0,
        }
    }
}

impl Mapper for Mapper3 {
    fn read_prg_byte(&self, address: u16) -> u8 {
        match address {
            0x8000..=0xBFFF => self
                .data
                .prg_rom
                .read(Page::First(PageSize::SixteenKB), address - 0x8000),
            0xC000..=0xFFFF => self
                .data
                .prg_rom
                .read(Page::Last(PageSize::SixteenKB), address - 0xC000),
            a => panic!("bad address: {:04X}", a),
        }
    }

    fn write_prg_byte(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0xFFFF => {
                self.chr_0 = value as usize;
            }
            _ => (),
        }
    }

    fn read_chr_byte(&self, address: u16) -> u8 {
        self.data
            .chr_rom
            .read(Page::Number(self.chr_0, PageSize::EightKB), address)
    }

    fn write_chr_byte(&mut self, _: u16, _: u8) {}

    fn mirroring(&self) -> Mirroring {
        self.data.header.mirroring
    }
}
