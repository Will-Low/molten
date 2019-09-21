#[macro_use]
extern crate molten;

// For processing
#[molten]
fn test_function(arg1: usize, arg2: usize) {
    println!("{}, something, {}", arg1, arg2);
}

fn main() {
   test_function(1, 2); 
}

