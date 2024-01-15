
// SystemBus is a virtual bus that connects all the components of the system
// 1. CPU cycle sync
// 2. Memory mapping
// 3. Interrupt handling
// 4. APU
// 5. PPU
// 6. Controllers
pub struct SystemBus {
    // CPU cycle sync
    pub cycles: u64,
    // CPU stall cycles
    stall_cycles: u8,
}

impl SystemBus {
    pub fn new()-> Self {
        Self {
            cycles: 0,
            stall_cycles: 0,
        }
    }

    pub fn tick(&mut self){
        self.cycles+=1;
        let cycles=self.cycles;
        // TODO: sync with APU
        
        // SYNC with nmi

        // TODO: sync with PPU, 3 PPU ticks per CPU cycle

    }
}