use std::{collections::HashMap, cell::RefCell};

use strum::IntoEnumIterator;

use super::{register::{Register, RegisterName, RegisterData}, flags::Flags};

pub struct ExecutingContext {
    /// Tracks the state of the flags register
    pub flags: Flags,

    /// Maps the register names to their data
    pub registers: HashMap<RegisterName, RefCell<Register>>,
    
    /// Stores the stack memory
    pub stack: Vec<RegisterData>,

    /// Stores the heap memory
    pub heap: Vec<Vec<u8>>,

    /// Maps identifiers to their pointers on the heap
    pub symtab: HashMap<String, i32>,

    /// Maps the label names to their pointers
    pub labels: HashMap<String, usize>,

    /// Current instruction pointer
    pub ptr: usize,

    /// Next instruction pointer
    pub next: usize,
}

impl ExecutingContext {
    pub fn new() -> Self {
        let registers: HashMap<RegisterName, RefCell<Register>> =
            HashMap::from_iter(RegisterName::iter().map(|r| {
                let reg = match r {
                    RegisterName::Eip |
                    RegisterName::Esp => Register { data: RegisterData::Pointer(0) },

                    // Offset by 1 to account for final return address as first stack value
                    RegisterName::Ebp => Register { data: RegisterData::Pointer(1) },
                    
                    _ => Register::new(),
                };

                (r, RefCell::from(reg))
            }));


        Self {
            stack: Vec::new(),
            flags: Flags::new(),
            registers,
            heap: Vec::new(),
            symtab: HashMap::new(),
            labels: HashMap::new(),
            ptr: 0,
            next: 1,
        }
    }
}
