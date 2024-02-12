use super::Mirroring;
use std::ops::Range;

const PRG_ROM_PAGE_SIZE: usize = 0x4000;
const PRG_RAM_PAGE_SIZE: usize = 0x2000;
const CHR_ROM_PAGE_SIZE: usize = 0x2000;
const CHR_RAM_PAGE_SIZE: usize = 0x2000;

#[derive(Copy, Clone)]
pub struct Header {
    pub mapper_number: u8,
    pub mirroring: Mirroring,
    pub prg_rom_pages: usize,
    pub prg_ram_pages: usize,
    pub chr_rom_pages: usize,
    pub preamable: bool,
}

impl Header {
    pub fn new(data: &[u8]) -> Self {
        Header {
            preamable: data[0..4] == [0x4E, 0x45, 0x53, 0x1A],
            mirroring: if data[6] & 0x01 == 0 {
                Mirroring::Horizontal
            } else {
                Mirroring::Vertical
            },
            prg_rom_pages: data[4] as usize,
            prg_ram_pages: if data[8] == 0 { 1 } else { data[8] } as usize,
            chr_rom_pages: data[5] as usize,
            mapper_number: (data[6] >> 4) | (data[7] & 0xF0),
        }
    }

    pub fn prg_rom_range(&self) -> Range<usize> {
        16..16 + self.prg_rom_bytes()
    }

    pub fn chr_rom_range(&self) -> Range<usize> {
        let prg_range = self.prg_rom_range();
        prg_range.end..prg_range.end + self.chr_rom_bytes()
    }

    pub fn prg_rom_bytes(&self) -> usize {
        self.prg_rom_pages * PRG_ROM_PAGE_SIZE
    }

    pub fn prg_ram_bytes(&self) -> usize {
        self.prg_ram_pages * PRG_RAM_PAGE_SIZE
    }

    pub fn chr_rom_bytes(&self) -> usize {
        self.chr_rom_pages * CHR_ROM_PAGE_SIZE
    }

    pub fn chr_ram_bytes(&self) -> usize {
        if self.chr_rom_pages == 0 {
            CHR_RAM_PAGE_SIZE
        } else {
            0
        }
    }
}
