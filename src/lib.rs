#![crate_type = "lib"]
#![crate_name = "synterm"]
extern crate strum;
#[macro_use]
extern crate strum_macros;


/// This will split the input into tokens to parse later
pub fn split_tokens(current_line: &str) -> Vec<&str> {
    return current_line.split_whitespace().collect();
}

///[WIP] Lexer Structure
pub struct Lexer {
    items: std::collections::HashMap<Token, &'static str>,
}

#[derive(Display, std::hash::Hash, std::cmp::Eq, std::cmp::PartialEq)]
/// Add tokens enumeration
pub enum Token {
    #[strum(serialize = "Number")]
    Number,
    #[strum(serialize = "Operator")]
    Operator,
    #[strum(serialize = "Comment")]
    Comment,
    #[strum(serialize = "Keyword")]
    Keyword,
}

impl Lexer {
    /// [WIP] This will add syntax highlighting for the termianl
    pub fn syntax_terminal(the_lexer: Lexer) {
        // WIP
    }
    pub fn new() -> Lexer {
        Lexer{
            items: std::collections::HashMap::new(),
        }
    }
}
