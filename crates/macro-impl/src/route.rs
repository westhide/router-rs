use syn::{
    __private::{
        Span, TokenStream2,
        quote::{ToTokens, quote},
    },
    Ident,
};

use crate::{attr::Attr, item::Item};

#[derive(Debug)]
pub struct Route {
    pub attr: Attr,
    pub item: Item,
}

impl Route {
    pub const fn new(attr: Attr, item: Item) -> Self {
        Self { attr, item }
    }

    pub fn route_name(&self) -> Ident {
        let ident = self.item.function_name();
        let fn_name = format!("__router_macro_{ident}");
        Ident::new(&fn_name, Span::call_site())
    }

    pub fn gen_route(&self) -> TokenStream2 {
        let method = &self.attr.method;
        let handler = self.item.function_name();
        let route_name = self.route_name();

        let http = quote!(::router_macro::core::http);
        quote! {
            pub fn #route_name<S>() -> #http::MethodRouter<S>
            where
                S: Clone + Send + Sync + 'static
            {
                #http::on(#http::Method::#method, #handler)
            }
        }
    }
}

impl ToTokens for Route {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { item, .. } = self;
        let gen_route = self.gen_route();

        let token = quote! {
            #item
            #gen_route
        };
        tokens.extend(token);
    }
}
