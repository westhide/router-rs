use syn::{
    __private::{TokenStream2, quote::ToTokens},
    Ident, ItemFn, Result,
    parse::{Parse, ParseStream},
};

#[derive(Debug)]
pub struct Item {
    pub inner: ItemFn,
}

impl Item {
    pub const fn new(inner: ItemFn) -> Self {
        Self { inner }
    }

    pub const fn function_name(&self) -> &Ident {
        &self.inner.sig.ident
    }
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner = input.parse::<ItemFn>()?;
        Ok(Self::new(inner))
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.inner.to_tokens(tokens)
    }
}
