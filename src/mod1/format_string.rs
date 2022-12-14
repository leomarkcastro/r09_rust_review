
pub fn with_zeroes(number: i32, zero_pads: usize) -> String {
    format!("{0:0>width$}", number, width = zero_pads as usize)
}

pub fn to_hex(number: i32) -> String {
    format!("{:X}", number)
}
