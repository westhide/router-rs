use rust_router::handler;

#[handler(Put)]
fn a() {
    println!("a");
    println!("a");
}

fn main() {
    a();
}
