extern crate syn;
extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::Item;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn molten(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(input).expect("failed to parse input");

    match item {
        Item::Fn(ref fn_item) => {
            println!("{:?}", fn_item); 
        },
        _ => {
            item.span().unstable(); 
        }
    }

    let output = quote!{ #item };
    output.into()
}
