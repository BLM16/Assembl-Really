use strum::EnumString;

/// https://www.tutorialspoint.com/assembly_programming/assembly_registers.htm
#[derive(Debug, PartialEq)]
#[derive(EnumString)]
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
