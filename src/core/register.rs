use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, ShlAssign, ShrAssign, BitAndAssign, BitOrAssign, BitXorAssign};

use strum::{EnumString, EnumIter};

#[derive(Clone, Copy)]
pub struct Register {
    pub data: RegisterData
}

#[derive(Clone, Copy, Debug)]
pub enum RegisterData {
    Value(i32),
    Pointer(i32),
}

/// https://www.tutorialspoint.com/assembly_programming/assembly_registers.htm
#[derive(Debug, Eq, Hash, PartialEq)]
#[derive(EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum RegisterName {
// Data
    Eax,
    Ebx,
    Ecx,
    Edx,

// Pointers
    Eip,
    Esp,
    Ebp,
}

impl Register {
    pub fn new() -> Self {
        Register { data: RegisterData::Value(0) }
    }

    pub fn get_raw(&self) -> i32 {
        match self.data {
            RegisterData::Value(i) => i,
            RegisterData::Pointer(p) => p,
        }
    }
}


impl AddAssign<i32> for Register {
    fn add_assign(&mut self, rhs: i32) {
        match self.data {
            RegisterData::Value(i) => self.data = RegisterData::Value(i + rhs),
            RegisterData::Pointer(p) => self.data = RegisterData::Pointer(p + rhs),
        }
    }
}

impl AddAssign for Register {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(rhs.get_raw())
    }
}

impl SubAssign<i32> for Register {
    fn sub_assign(&mut self, rhs: i32) {
        match self.data {
            RegisterData::Value(i) => self.data = RegisterData::Value(i - rhs),
            RegisterData::Pointer(p) => self.data = RegisterData::Pointer(p - rhs),
        }
    }
}

impl SubAssign for Register {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign(rhs.get_raw())
    }
}

impl MulAssign<i32> for Register {
    fn mul_assign(&mut self, rhs: i32) {
        match self.data {
            RegisterData::Value(i) => self.data = RegisterData::Value(i * rhs),
            RegisterData::Pointer(p) => self.data = RegisterData::Pointer(p * rhs),
        }
    }
}

impl MulAssign for Register {
    fn mul_assign(&mut self, rhs: Self) {
        self.mul_assign(rhs.get_raw())
    }
}

impl DivAssign<i32> for Register {
    fn div_assign(&mut self, rhs: i32) {
        match self.data {
            RegisterData::Value(i) => self.data = RegisterData::Value(i / rhs),
            RegisterData::Pointer(p) => self.data = RegisterData::Pointer(p / rhs),
        }
    }
}

impl DivAssign for Register {
    fn div_assign(&mut self, rhs: Self) {
        self.div_assign(rhs.get_raw())
    }
}

impl ShlAssign<i32> for Register {
    fn shl_assign(&mut self, rhs: i32) {
        match self.data {
            RegisterData::Value(i) => self.data = RegisterData::Value(i << rhs),
            RegisterData::Pointer(p) => self.data = RegisterData::Pointer(p << rhs),
        }
    }
}

impl ShrAssign<i32> for Register {
    fn shr_assign(&mut self, rhs: i32) {
        match self.data {
            RegisterData::Value(i) => self.data = RegisterData::Value(i >> rhs),
            RegisterData::Pointer(p) => self.data = RegisterData::Pointer(p >> rhs),
        }
    }
}

impl BitAndAssign<i32> for Register {
    fn bitand_assign(&mut self, rhs: i32) {
        match self.data {
            RegisterData::Value(i) => self.data = RegisterData::Value(i & rhs),
            RegisterData::Pointer(p) => self.data = RegisterData::Pointer(p & rhs),
        }
    }
}

impl BitAndAssign for Register {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bitand_assign(rhs.get_raw())
    }
}

impl BitOrAssign<i32> for Register {
    fn bitor_assign(&mut self, rhs: i32) {
        match self.data {
            RegisterData::Value(i) => self.data = RegisterData::Value(i | rhs),
            RegisterData::Pointer(p) => self.data = RegisterData::Pointer(p | rhs),
        }
    }
}

impl BitOrAssign for Register {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bitor_assign(rhs.get_raw())
    }
}

impl Register {
    pub fn bitnot_assign_self(&mut self) {
        match self.data {
            RegisterData::Value(i) => self.data = RegisterData::Value(!i),
            RegisterData::Pointer(p) => self.data = RegisterData::Pointer(!p),
        }
    }
}

impl BitXorAssign<i32> for Register {
    fn bitxor_assign(&mut self, rhs: i32) {
        match self.data {
            RegisterData::Value(i) => self.data = RegisterData::Value(i ^ rhs),
            RegisterData::Pointer(p) => self.data = RegisterData::Pointer(p ^ rhs),
        }
    }
}

impl BitXorAssign for Register {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bitxor_assign(rhs.get_raw())
    }
}
