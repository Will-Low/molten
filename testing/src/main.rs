extern crate syn;

use std::fs::File;
use std::io::prelude::*;
use syn::*;
use syn::parse::*;


macro_rules! molten {
    ($x:expr) => {
        let mut file = File::open($x).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let stream = syn::parse_str::<TypeInfer>(&contents);
        println!("{:?}", syn::parse::Parse::parse(stream.unwrap().underscore_token));
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

