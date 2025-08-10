use std::str::FromStr;

use syn::{
    __private::{
        TokenStream2,
        quote::{ToTokens, quote},
    },
    Error, Ident, Result,
    parse::{Parse, ParseStream},
};

use crate::error::err;

#[derive(Debug)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

impl FromStr for Method {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let method = match s {
            "Get" => Self::Get,
            "Head" => Self::Head,
            "Post" => Self::Post,
            "Put" => Self::Put,
            "Delete" => Self::Delete,
            "Connect" => Self::Connect,
            "Options" => Self::Options,
            "Trace" => Self::Trace,
            "Patch" => Self::Patch,
            s => return err!("Invalid Method: {s}"),
        };
        Ok(method)
    }
}

impl Parse for Method {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            let this = ident.to_string().parse()?;
            Ok(this)
        } else {
            Err(input.error("Invalid Method Ident"))
        }
    }
}

impl ToTokens for Method {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let token = match self {
            Self::Get => quote! {::rust_router::restfull::GET},
            Self::Head => quote! {::rust_router::restfull::HEAD},
            Self::Post => quote! {::rust_router::restfull::POST},
            Self::Put => quote! {::rust_router::restfull::PUT},
            Self::Delete => quote! {::rust_router::restfull::DELETE},
            Self::Connect => quote! {::rust_router::restfull::CONNECT},
            Self::Options => quote! {::rust_router::restfull::OPTIONS},
            Self::Trace => quote! {::rust_router::restfull::TRACE},
            Self::Patch => quote! {::rust_router::restfull::PATCH},
        };
        tokens.extend(token);
    }
}

#[derive(Debug)]
pub struct Attr {
    pub method: Method,
}

impl Attr {
    pub const fn new(method: Method) -> Self {
        Self { method }
    }
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> Result<Self> {
        let method = input.parse()?;
        // TODO: parse args
        Ok(Self::new(method))
    }
}
