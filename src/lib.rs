#![crate_type = "lib"]
#![crate_name = "synterm"]

/// This struct defines a highlighting pair
pub struct HighlightingPair<'t> {
    item: &'t str,
    color: &'t str,
}

/// This will split the input into tokens to parse later
pub fn split_tokens(current_line: &str) -> Vec<&str> {
    return current_line.split(" ").collect();
}

/// Add tokens enumeration
pub enum Tokens {
    Number,
    Operator,
    Comment,
    Keyword,
}
