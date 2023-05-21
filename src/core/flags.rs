pub enum Flag {
    CF = 0b1,            // Carry
    ZF = 0b1000000,      // Zero
    SF = 0b10000000,     // Sign
    OF = 0b100000000000, // Overflow
}

pub struct Flags {
    data: u32,
}

impl Flags {
    pub fn new() -> Self {
        Flags { data: 0 }
    }

    pub fn get(&self, flag: Flag) -> bool {
        self.data & flag as u32 != 0
    }

    pub fn set(&mut self, flag: Flag) {
        self.data |= flag as u32;
    }

    pub fn unset(&mut self, flag: Flag) {
        self.data &= !(flag as u32);
    }
}
