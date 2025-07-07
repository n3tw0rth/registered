use std::env;

use registered::registers::REGISTERS;

fn main() {
    let args: Vec<String> = env::args().collect();

    let register_name = args.get(1).expect("Please enter the register name");

    let str_value = args
        .get(2)
        .map(|v| v.strip_prefix("0x").expect("Does not contain 0x prefix"))
        .expect("Might not a valid hex value");

    let registers: String = REGISTERS
        .iter()
        .map(|v| {
            print!("{:?}", v);
            v.join(",")
        })
        .collect();

    let decimal_value = u64::from_str_radix(str_value, 16);

    println!("{:?}", registers);
}
