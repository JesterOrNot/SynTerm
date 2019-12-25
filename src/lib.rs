#![crate_type = "lib"]
#![crate_name = "synterm"]
extern crate strum;
#[macro_use]
extern crate strum_macros;

/// This struct defines a highlighting pair
pub struct HighlightingPair<'t> {
    pub token: Token,
    pub color: &'t str,
}

impl std::fmt::Display for HighlightingPair<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Token: {}, Color: {})", self.token, self.color)
    }
}

/// This will split the input into tokens to parse later
pub fn split_tokens(current_line: &str) -> Vec<&str> {
    return current_line.split(" ").collect();
}

///[WIP] Lexer Structure
pub struct Lexer {
    items: Vec<HighlightingPair<'static>>,
}

#[derive(Display)]
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
}
