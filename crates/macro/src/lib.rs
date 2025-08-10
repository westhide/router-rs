mod attr;
mod error;

use proc_macro::TokenStream;
use syn::{__private::quote::quote, parse};

use crate::attr::Attr;

#[proc_macro_attribute]
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse::<Attr>(attr).unwrap();

    let m = attr.method;
    println!("===={m:?}");

    let ret = quote! {
        // #item
        fn a() {
            #m;
        }
    };

    println!("ret===={ret}");
    ret.into()
}
