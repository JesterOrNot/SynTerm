#![crate_type = "lib"]
#![crate_name = "synterm"]

/// This struct defines a highlighting pair
pub struct HighlightingPair<'t> {
    token: Token,
    color: &'t str,
}

/// This will split the input into tokens to parse later
pub fn split_tokens(current_line: &str) -> Vec<&str> {
    return current_line.split(" ").collect();
}

///[WIP] Lexer Structure
pub struct Lexer {
    Items: Vec<HighlightingPair<'static>>,
}

/// Add tokens enumeration
pub enum Token {
    Number,
    Operator,
    Comment,
    Keyword,
}

///[WIP] This will add syntax highlighting for the termianl
pub fn SyntaxTerminal(the_lexer: Lexer) {
    // WIP
}
