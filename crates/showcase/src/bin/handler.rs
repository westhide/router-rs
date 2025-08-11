use router_macro::handler;

#[handler(Get)]
async fn get() -> String {
    format!("Main")
}

// async fn get_impl() -> String {
//     format!("Main")
// }
// pub fn __router_macro_get_impl<S>() -> ::router_macro::core::http::MethodRouter<S>
// where
//     S: Clone + Send + Sync + 'static,
// {
//     ::router_macro::core::http::on(::router_macro::core::http::Method::GET, get_impl)
// }

#[tokio::main]
async fn main() {
    let s = get().await;
    println!("{s}");
}
