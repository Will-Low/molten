extern crate proc_macro2;
extern crate syn;

use std::fs::File;
use std::io::prelude::*;

macro_rules! molten {
    ($x:expr) => {
        let mut file = File::open($x).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read file");
        let file_text = syn::parse_file(&contents).expect("Unable to parse file");
        println!("{:#?}", file_text.items);
    }
}

// For processing
fn sum(value1: i64, value2: i64) -> i64 {
    value1 + value2
}

fn subtract(value1: i64, value2: i64) -> i64 {
    value1 - value2
}

fn main() {

    molten!(file!());
    sum(13, 2); 
    subtract(3,2);
}

