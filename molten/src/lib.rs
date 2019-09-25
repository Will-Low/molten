extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use std::collections::HashMap;
use syn::FnArg::Typed;
use syn::Item;
use syn::Pat::Ident;

#[derive(Debug)]
struct FnInfo {
    fn_name: String,
    args: Vec<String>,
}

static mut DATA: Vec<FnInfo> = vec!();

#[proc_macro_attribute]
pub fn molten(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(input).expect("failed to parse input");

    match item {
        Item::Fn(ref fn_item) => {
            let fn_struct = FnInfo {
                fn_name: fn_item.sig.ident.to_string(),
                args: fn_item
                    .sig
                    .inputs
                    .iter()
                    .map(|input| {
                        if let Typed(typed) = input {
                            if let Ident(pat_ident) = &*typed.pat {
                                let arg_name = pat_ident.ident.to_string();
                                arg_name
                            } else {
                                eprintln!("No argument found");
                                String::from("None")
                            }
                        } else {
                            eprintln!("No function data found");
                            String::from("None")
                        }
                    })
                    .collect(),
            };
            unsafe {
                DATA.push(fn_struct);
            }
        }
        _ => {
            eprintln!("Type not currently supported.");
        }
    }
    println!("{:#?}", DATA);
    let output = quote! { #item };
    output.into()
}

