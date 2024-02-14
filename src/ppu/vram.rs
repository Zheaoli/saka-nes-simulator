use crate::cartridge::{Cartridge, Mirroring};
use std::cell::RefCell;
use std::rc::Rc;

const NAMETABLE_SIZE: usize = 0x400;
const PALETTE_SIZE: usize = 0x20;

pub struct Vram {
    pub nametables: [u8; 2 * NAMETABLE_SIZE],
    pub palette: [u8; PALETTE_SIZE],
    read_buffer: u8,
    cartridge: Option<Rc<RefCell<Cartridge>>>,
}

impl Vram {
    pub fn new() -> Self {
        Vram {
            nametables: [0; 2 * NAMETABLE_SIZE],
            palette: [0; PALETTE_SIZE],
            read_buffer: 0,
            cartridge: None,
        }
    }

    pub fn reset(&mut self) {
        self.nametables = [0xFF; 0x800];
        self.palette = [0; 0x20];
        self.read_buffer = 0;
        self.cartridge = None;
    }

    pub fn set_cartridge(&mut self, cartridge: Rc<RefCell<Cartridge>>) {
        self.cartridge = Some(cartridge);
    }

    pub fn mirroring(&self) -> Mirroring {
        if let Some(ref c) = self.cartridge {
            c.borrow().mirroring()
        } else {
            Mirroring::None
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        let mirroring = self.mirroring();
        match address {
            0x0000..=0x1FFF => {
                if let Some(ref c) = self.cartridge {
                    c.borrow_mut().write_chr_byte(address, value);
                }
            }
            0x2000..=0x3EFF => self.nametables[mirror_nametable(mirroring, address)] = value,
            0x3F00..=0x3FFF => self.palette[mirror_palette(address)] = value,
            _ => (),
        };
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        let mirroring = self.mirroring();
        match address {
            0x0000..=0x1FFF => match self.cartridge {
                Some(ref c) => c.borrow().read_chr_byte(address),
                None => panic!("tried to read non-existant cartridge memory"),
            },
            0x2000..=0x3EFF => self.nametables[mirror_nametable(mirroring, address)],
            0x3F00..=0x3FFF => self.palette[mirror_palette(address)],
            _ => 0,
        }
    }

    pub fn buffered_read_byte(&mut self, address: u16) -> u8 {
        if address < 0x3F00 {
            let value = self.read_buffer;
            self.read_buffer = self.read_byte(address);
            value
        } else {
            let mirroring = self.mirroring();
            self.read_buffer = self.nametables[mirror_nametable(mirroring, address)];
            self.read_byte(address)
        }
    }
}

fn mirror_palette(address: u16) -> usize {
    let address = (address as usize) % PALETTE_SIZE;
    match address {
        0x10 | 0x14 | 0x18 | 0x1C => address - 0x10,
        _ => address,
    }
}

fn mirror_nametable(mirroring: Mirroring, address: u16) -> usize {
    let address = address as usize;
    match mirroring {
        Mirroring::None => address - 0x2000,
        Mirroring::Horizontal => ((address / 2) & NAMETABLE_SIZE) + (address % NAMETABLE_SIZE),
        Mirroring::Vertical => address % (2 * NAMETABLE_SIZE),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_mirror_nametable_horizontally() {
        // Nametable 1 - starting at 0x2000
        assert_eq!(mirror_nametable(Mirroring::Horizontal, 0x2801), 0x401);
        assert_eq!(mirror_nametable(Mirroring::Horizontal, 0x2A01), 0x601);
        assert_eq!(mirror_nametable(Mirroring::Horizontal, 0x2C01), 0x401);
        assert_eq!(mirror_nametable(Mirroring::Horizontal, 0x2E01), 0x601);
    }
}
