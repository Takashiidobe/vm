use std::fmt;

use Reg::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reg {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = match self {
            R0 => "R0",
            R1 => "R1",
            R2 => "R2",
            R3 => "R3",
            R4 => "R4",
            R5 => "R5",
            R6 => "R6",
            R7 => "R7",
            R8 => "R8",
            R9 => "R9",
            R10 => "R10",
            R11 => "R11",
            R12 => "R12",
            R13 => "R13",
            R14 => "R14",
            R15 => "R15",
        };
        f.write_str(r)
    }
}

impl From<&str> for Reg {
    fn from(value: &str) -> Self {
        match value {
            "R0" => R0,
            "R1" => R1,
            "R2" => R2,
            "R3" => R3,
            "R4" => R4,
            "R5" => R5,
            "R6" => R6,
            "R7" => R7,
            "R8" => R8,
            "R9" => R9,
            "R10" => R10,
            "R11" => R11,
            "R12" => R12,
            "R13" => R13,
            "R14" => R14,
            "R15" => R15,
            _ => panic!("Could not parse {value}"),
        }
    }
}

impl From<Reg> for u8 {
    fn from(val: Reg) -> Self {
        match val {
            R0 => 0,
            R1 => 1,
            R2 => 2,
            R3 => 3,
            R4 => 4,
            R5 => 5,
            R6 => 6,
            R7 => 7,
            R8 => 8,
            R9 => 9,
            R10 => 11,
            R11 => 12,
            R12 => 13,
            R13 => 14,
            R14 => 15,
            R15 => 16,
        }
    }
}

impl From<u8> for Reg {
    fn from(value: u8) -> Self {
        match value {
            0 => R0,
            1 => R1,
            2 => R2,
            3 => R3,
            4 => R4,
            5 => R5,
            6 => R6,
            7 => R7,
            8 => R8,
            9 => R9,
            11 => R10,
            12 => R11,
            13 => R12,
            14 => R13,
            15 => R14,
            16 => R15,
            _ => panic!("Could not convert u8 to Reg"),
        }
    }
}
