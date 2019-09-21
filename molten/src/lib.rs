extern crate syn;
extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::FnArg::Typed;
use syn::Item;
use syn::Pat::Ident;

struct FnInfo<'a, T: 'a> {
    fn_name: String,
    args: Vec<Arg<'a, T>>
}

struct Arg<'a, T: 'a> {
    arg_name: &'a str,
    values: Vec<T>
}

#[proc_macro_attribute]
pub fn molten(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(input).expect("failed to parse input");
    match item {
        Item::Fn(ref fn_item) => {
            let fn_struct = FnInfo {
                                fn_name: fn_item.sig.ident.to_string(),
                                args: fn_item.sig.inputs.iter().map(|input| 
                                    if let Typed(typed) = input {
                                        let pat = typed.pat;
                                        if let Ident(pat_ident) = *pat {
                                            let arg_name = pat_ident.ident.to_string();
                                        }
                                    }).collect()
                                };
            println!("{}", fn_item.sig.ident.to_string()); 
        },
        _ => {
            eprintln!("Type not currently supported.");
        }
    }

    let output = quote!{ #item };
    output.into()
}
