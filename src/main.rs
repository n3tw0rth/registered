use registered::registers::{BitSize, RegResult, RegResultGroup, Register, RegisterGroup};
use std::env;
use tabled::Table;

use registered::registers::REGISTERS;

fn main() {
    let args: Vec<String> = env::args().collect();

    let register_name = args.get(1).expect("Please enter the register name");

    let str_value = args
        .get(2)
        .map(|v| v.strip_prefix("0x").expect("Does not contain 0x prefix"))
        .expect("Might not a valid hex value");

    let registers_group: &RegisterGroup = REGISTERS
        .iter()
        .find(|r| r.variants.iter().any(|v| v.name == register_name))
        .expect("Failed to find the correct register group");

    let decimal_value = u64::from_str_radix(str_value, 16);
    let binary_string = format!("{:0>64b}", decimal_value.unwrap());

    let variants = map_values_to_registers(registers_group, binary_string).variants;

    let table = Table::new(variants);

    print!("{table}")
}

fn map_values_to_registers(
    register_group: &RegisterGroup,
    binary_string: String,
) -> RegResultGroup {
    let mut variants: Vec<RegResult> = Vec::new();

    register_group.variants.iter().for_each(|r| match r.size {
        BitSize::Bit8 => {
            variants.push(process(8, binary_string.clone(), r.clone()));
        }
        BitSize::Bit16 => {
            variants.push(process(16, binary_string.clone(), r.clone()));
        }
        BitSize::Bit32 => {
            variants.push(process(32, binary_string.clone(), r.clone()));
        }
        BitSize::Bit64 => {
            variants.push(process(64, binary_string.clone(), r.clone()));
        }
    });
    RegResultGroup { variants }
}

fn process(size: usize, binary_string: String, r: Register) -> RegResult {
    let slice = binary_string.split_at(64 - size).1;
    let decimal_value = u64::from_str_radix(slice, 2).unwrap_or_default();
    let hex_value = format!("0x{:x}", decimal_value);
    RegResult {
        name: r.name,
        size: r.size,
        decimal: decimal_value,
        hex: hex_value,
    }
}
