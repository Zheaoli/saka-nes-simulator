use std::intrinsics::offset;

// 7  bit  0
// ---- ----
// NVss DIZC
// |||| ||||
// |||| |||+- Carry
// |||| ||+-- Zero
// |||| |+--- Interrupt Disable
// |||| +---- Decimal
// ||++------ No CPU effect, see: the B flag
// |+-------- Overflow
// +--------- Negative
mod utils;
enum FlagBit {
    Carry = 0b00000001,
    Zero = 0b00000010,
    IrqDisable = 0b00000100,
    Decimal = 0b00001000,
    Break = 0b00010000,
    Push = 0b00100000,
    Overflow = 0b01000000,
    Negative = 0b10000000,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Mode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    NoMode,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum InterruptType {
    NMI,
    IRQ,
    BRK,
    None,
}

pub struct CPU {
    // TODO: Add bus here
    pc: u16,
    sp: u8,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            p: 0,
        }
    }

    pub fn reset_registers(&mut self) {
        self.sp = 0xFF;
        self.p = 0x34;
        // TODO: set reset interrupt
    }

    pub fn pop_byte(&mut self) -> u8 {
        // Address range 0x0100-0x01FF
        self.sp = self.sp.wrapping_add(1);
        let address = 0x0100 + (self.sp as u16);
        // TODO: read from bus
        0
    }

    pub fn push_byte(&mut self, data: u8) {
        // Address range 0x0100-0x01FF
        let address = 0x0100 + (self.sp as u16);
        // TODO: write to bus
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn push_2bytes(&mut self, data: u16) {
        // Address range 0x0100-0x01FF
        self.push_byte((data >> 8) as u8);
        self.push_byte(data as u8);
    }

    pub fn pop_2bytes(&mut self) -> u16 {
        (self.pop_byte() as u16) | ((self.pop_byte() as u16) << 8)
    }

    pub fn incrase_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }

    pub fn next_byte(&mut self) -> u8 {
        let address = self.pc;
        self.incrase_pc();
        // TODO: read from bus
        0
    }

    pub fn next_2bytes(&mut self) -> u16 {
        let address = self.pc;
        self.incrase_pc();
        self.incrase_pc();
        // TODO: read from bus
        0
    }

    fn check_flag(&self, flag: FlagBit) -> bool {
        (self.p & (flag as u8)) != 0
    }

    fn update_flag(&mut self, flag: FlagBit, value: bool) {
        if value {
            self.p |= flag as u8;
        } else {
            self.p &= !(flag as u8);
        }
    }

    fn update_zero_and_negative(&mut self, value: u8) {
        self.update_flag(FlagBit::Zero, value == 0);
        self.update_flag(FlagBit::Negative, (value & 0b10000000) != 0);
    }

    fn update_flow_carry_overflow(&mut self, a: u8, b: u8, result: u16) {
        self.update_flag(FlagBit::Carry, result > 0xFF);
        let temp = result as u8;
        self.update_flag(FlagBit::Overflow, (a ^ temp) & (b ^ temp) & 0x80 != 0);
    }

    fn update_carry_only(&mut self, result: u16) {
        self.p & (FlagBit::Carry as u8);
    }

    fn get_oeratate_address(&self, mode: Mode) -> u16 {
        match mode {
            Mode::Immediate => {
                let temp = self.pc;
                self.incrase_pc();
                temp
            }
            Mode::ZeroPage => self.next_byte() as u16,
            Mode::ZeroPageX => {
                // TODO: 1 CPU Tick
                utils::low_byte(utils::offset(self.next_byte(), self.x))
            }
            Mode::ZeroPageY => {
                // TODO: 1 CPU Tick
                utils::low_byte(utils::offset(self.next_byte(), self.y))
            }
            Mode::Absolute => self.next_2bytes(),
            Mode::AbsoluteX => {
                let temp = self.next_2bytes();
                if utils::check_cross_page(temp, self.x) {
                    // TODO: 1 CPU Tick
                }
                utils::offset(temp, self.x)
            }
            Mode::AbsoluteY => {
                let temp = self.next_2bytes();
                if utils::check_cross_page(temp, self.y) {
                    // TODO: 1 CPU Tick
                }
                utils::offset(temp, self.y)
            }
            Mode::Indirect => {
                let temp = self.next_2bytes();
                // TODO: Read from bus
                0
            }
            Mode::IndirectX => {
                // TODO: 1 CPU Tick
                let temp = self.next_byte();
                let address = utils::offset(temp, self.x);
                // TODO: Read from bus
                0
            }
            Mode::IndirectY=>{
                let temp = self.next_byte();
                // TODO: Read from bus
                let base=0 as u16;
                if utils::check_cross_page(base, self.y) {
                    // TODO: 1 CPU Tick
                }
                utils::offset(base, self.y)
            }
            Mode::NoMode => panic!("No mode"),
        }
    }
}
