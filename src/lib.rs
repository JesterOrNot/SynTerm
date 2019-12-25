//! This struct defines a highlighting pair
struct HighlightingPair<'T> {
    item: &'T str,
    color: &'T str,
}

///! This will split the input into tokens to parse later
fn split_tokens(current_line: &str) -> Vec<&str> {
    return current_line.split(" ").collect();
}

///! Add tokens enumeration
enum Tokens {
    Number,
    Operator,
    Comment,
    Keyword,
}
