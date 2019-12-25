extern crate synterm;

fn main() {
    let pair = synterm::HighlightingPair{color: "Blue", token: synterm::Token::Operator};
    println!("{}", pair);
}