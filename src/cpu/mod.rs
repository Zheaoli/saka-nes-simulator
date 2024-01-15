use log::debug;

mod debug;
mod utils;

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
    RESET,
    BRK,
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

    fn get_operand_address(&self, mode: Mode) -> u16 {
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
            Mode::IndirectY => {
                let temp = self.next_byte();
                // TODO: Read from bus
                let base = 0 as u16;
                if utils::check_cross_page(base, self.y) {
                    // TODO: 1 CPU Tick
                }
                utils::offset(base, self.y)
            }
            Mode::NoMode => panic!("No mode"),
        }
    }

    fn fetch_operand(&mut self, mode: Mode) -> u8 {
        let address = self.get_operand_address(mode);
        // TODO: Read from bus
        0
    }

    fn interrupt(&mut self, kind: InterruptType) {
        let (ticks, push, address, flags) = match kind {
            InterruptType::NMI => (2, true, 0xFFFAu16, vec![FlagBit::IrqDisable]),
            InterruptType::RESET => (5, false, 0xFFFCu16, vec![]),
            InterruptType::IRQ => (2, true, 0xFFFEu16, vec![FlagBit::IrqDisable]),
            InterruptType::BRK => (
                1,
                true,
                0xFFFEu16,
                vec![FlagBit::IrqDisable, FlagBit::Break],
            ),
        };
        // Interrupts need couple of ticks to complete
        // The baseline is the BRK instruction, which takes 6 ticks
        // The rest of the interrupts take from 7-10 ticks
        for _ in 0..ticks {
            // TODO: 1 CPU Tick
        }
        // Push PC to stack
        if push {
            let pc = self.pc;
            // Update the state register and push it into the stack
            let mut p = self.p | FlagBit::Push as u8;
            if let InterruptType::BRK = kind {
                p |= FlagBit::Break as u8;
            }
            self.push_2bytes(pc);
            self.push_byte(p);
        }
        for f in flags {
            self.update_flag(f, true);
        }
        // TODO: read the new address
        self.pc = 0;
    }

    #[allow(dead_code)]
    pub fn log_instruction(&mut self) {
        let pc = self.pc;
        let rom_offset = 15 + (self.pc % 0x4000);
        // TODO: read from bus
        let opcode = 0 as usize;
        let mut args = String::new();
        for i in 1..debug::INSTRUCTION_SIZES[opcode] {

            // write!(args,"{:02X} ",self.fetch_operand(Mode::Immediate)).unwrap();
        }
        debug!("{}",format!("OFFSET: {:06x}\tPC: {:04x}\tOPCODE: {:02x}\tPC: {:04x}\tA: {:02x}\tX: {:02x}\tY: {:02x}\tP: {:08b}\t opcode: [{:02x}] {}\t{}",rom_offset,pc,opcode,pc,self.a,self.x,self.y,self.p,opcode,debug::INSTRUCTION_NAMES[opcode],args));
    }
}
