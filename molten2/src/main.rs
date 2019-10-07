extern crate clap;
extern crate proc_macro2;
extern crate syn;

use clap::{Arg, App, SubCommand};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use syn::FnArg::Typed;
use syn::Pat::Ident;

#[derive(Debug)]
struct Func {
    name: String,
    args: Vec<String>,
}

macro_rules! molten {
    ($x:expr, $f:expr) => {
        let matches = App::new(file!());
        
        let mut file = File::open($x).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read file");
        let file_text = syn::parse_file(&contents).expect("Unable to parse file");
        let mut functions: Vec<Func> = vec!();
        for item in file_text.items {
            let mut func = Func {name: String::from(""), args: vec!()}; 
            match item {
                syn::Item::Fn(item) => {
                    func.name = item.sig.ident.to_string();
                    for input in item.sig.inputs {
                        if let Typed(pat_type) = input {
                            if let Ident(pat_ident) = *pat_type.pat {
                                let value = pat_ident.ident.to_string();
                                func.args.push(value);
                            }
                        }
                    
                    }
                    functions.push(func);
                },
                _ => ()

            }
        }
        
        let mut subcommands: Vec<App> = vec!();
        for function in &functions {
            let mut args: Vec<Arg> = vec!(); 
            for arg in &function.args {
                args.push(Arg::with_name(arg).takes_value(true));
            }
            subcommands.push(SubCommand::with_name(&function.name).args(&args));
        }
        let matches = matches.subcommands(subcommands);
        

        matches.get_matches();
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
    molten!(file!(), sum);
}

