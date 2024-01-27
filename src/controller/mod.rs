#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Button {
    A = 0b0000_0001,
    B = 0b0000_0010,
    Select = 0b0000_0100,
    Start = 0b0000_1000,
    Up = 0b0001_0000,
    Down = 0b0010_0000,
    Left = 0b0100_0000,
    Right = 0b1000_0000,
}


pub struct Controller {
    pub buttons: u8,
    pub strobe: bool,
    pub index: u8,
}

impl Controller {
    pub fn new()->Self{
        Controller{
            buttons: 0,
            strobe: false,
            index: 0,
        }
    }

    pub fn write_register(&mut self, value: u8){
        self.strobe=value&0x01!=0;
        if self.strobe {
            self.index=0;
        }
    }

    pub fn read_register(&mut self)->u8{
        let value=if self.index<8{
            self.buttons>>self.index&1
        }else{
            1
        };
        if !self.strobe {
            self.index+=1;
        }
        0x40 | value
    }

    pub fn set_button(&mut self, button: Button, pressed: bool){
        self.buttons&=!(button as u8);
        if pressed {
            self.buttons|=button as u8;
        }
    }

    
}