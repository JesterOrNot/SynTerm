extern crate synterm;

fn main() {
    let split_tokens = synterm::split_tokens("Hello");
    println!("{:?}", split_tokens);
}