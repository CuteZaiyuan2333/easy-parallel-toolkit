// my_runner_macros/src/lib.rs
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(item as ItemStruct);
    let struct_name = &input_struct.ident;
    
    let expanded = quote! {
        #input_struct

        fn main() {
            let mut main_struct = #struct_name::init();
            main_struct.main();
        }
    };
    TokenStream::from(expanded)
}