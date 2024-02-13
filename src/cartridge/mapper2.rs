// Mapper2 implements ines mapper 2 (UxROM)
// https://wiki.nesdev.com/w/index.php/UxROM

use super::pager::Page;
use super::pager::PageSize;
use super::Data;
use super::Mapper;
use super::Mirroring;

pub struct Mapper2 {
    data: Data,
    prg_0: usize,
}

impl Mapper2 {
    pub fn new(data: Data) -> Self {
        Mapper2 {
            data: data,
            prg_0: 0,
        }
    }
}

impl Mapper for Mapper2 {
    fn read_prg_byte(&self, address: u16) -> u8 {
        match address {
            0x8000..=0xBFFF => self.data.prg_rom.read(
                Page::Number(self.prg_0, PageSize::SixteenKB),
                address - 0x8000,
            ),
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
                self.prg_0 = value as usize & 0x0F;
            }
            _ => panic!("bad address"),
        }
    }

    fn read_chr_byte(&self, address: u16) -> u8 {
        if self.data.header.chr_rom_pages == 0 {
            self.data
                .chr_ram
                .read(Page::First(PageSize::EightKB), address)
        } else {
            self.data
                .chr_rom
                .read(Page::First(PageSize::EightKB), address)
        }
    }

    fn write_chr_byte(&mut self, address: u16, value: u8) {
        if self.data.header.chr_rom_pages == 0 {
            self.data
                .chr_ram
                .write(Page::First(PageSize::EightKB), address, value)
        }
    }

    fn mirroring(&self) -> Mirroring {
        self.data.header.mirroring
    }
}
