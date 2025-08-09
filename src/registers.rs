use tabled::Tabled;

#[derive(Debug, Clone, Copy, strum_macros::Display, Tabled)]
pub enum BitSize {
    Bit64,
    Bit32,
    Bit16,
    Bit8,
}

#[derive(Debug, Clone, Tabled)]
pub struct Register {
    pub name: &'static str,
    pub size: BitSize,
}

#[derive(Debug, Clone)]
pub struct RegisterGroup {
    pub variants: [Register; 4],
}

#[derive(Debug, Tabled, Clone)]
pub struct RegResult {
    pub name: &'static str,
    pub size: BitSize,
    pub decimal: u64,
    pub hex: String,
}

#[derive(Debug)]
pub struct RegResultGroup {
    pub variants: Vec<RegResult>,
}

pub const REGISTERS: &[RegisterGroup] = &[
    RegisterGroup {
        variants: [
            Register {
                name: "rax",
                size: BitSize::Bit64,
            },
            Register {
                name: "eax",
                size: BitSize::Bit32,
            },
            Register {
                name: "ax",
                size: BitSize::Bit16,
            },
            Register {
                name: "al",
                size: BitSize::Bit8,
            },
        ],
    },
    RegisterGroup {
        variants: [
            Register {
                name: "rbx",
                size: BitSize::Bit64,
            },
            Register {
                name: "ebx",
                size: BitSize::Bit32,
            },
            Register {
                name: "bx",
                size: BitSize::Bit16,
            },
            Register {
                name: "bl",
                size: BitSize::Bit8,
            },
        ],
    },
];
