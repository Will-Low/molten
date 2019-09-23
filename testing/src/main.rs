#[macro_use]
extern crate molten;

// For processing
#[molten]
fn sum(value1: i64, value2: i64) -> i64 {
    value1 + value2
}

#[molten]
fn subtract(value1: i64, value2: i64) -> i64 {
    value1 - value2
}

fn main() {
    sum(13, 2); 
    subtract(3,2);
}

