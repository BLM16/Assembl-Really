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
    Ax,
    Bx,
    Cx,
    Dx,

// Pointers
    Eip,
    Esp,
    Ebp,
    Ip,
    Sp,
    Bp,

// Indicies
    Esi,
    Edi,
    Si,
    Di,

// Flags
    Flags,
}
