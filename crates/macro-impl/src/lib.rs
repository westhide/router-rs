mod attr;
mod error;
mod item;
mod route;

use proc_macro::TokenStream;
use syn::{
    __private::{TokenStream2, quote::quote},
    Result, parse,
};

use crate::{attr::Attr, item::Item, route::Route};

#[proc_macro_attribute]
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    match handler_impl(attr, item) {
        Ok(token) => token,
        Err(err) => err.into_compile_error(),
    }
    .into()
}

fn handler_impl(attr: TokenStream, item: TokenStream) -> Result<TokenStream2> {
    let attr = parse::<Attr>(attr)?;
    let item = parse::<Item>(item)?;
    let route = Route::new(attr, item);
    Ok(quote!(#route))
}
