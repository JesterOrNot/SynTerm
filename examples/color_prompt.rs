use std::process::exit;
use synterm::{gen_lexer, gen_parse, syntax_highlight_gen, Color, CommandLineTool};

struct MyTool;

impl CommandLineTool for MyTool {
    const PROMPT: &'static str = "\x01\x1b[1;33m\x02>>> \x01\x1b[m\x02";
    fn evaluator_function(line: &String) -> String {
        match line.as_str() {
            "exit" => {
                exit(0);
            }
            _ => format!("Line: {}", line),
        }
    }
    fn syntax_highlight(string: &str) {
        syntax_highlight_gen!(
            TheLexer,
            parser,
            (Red, Color::Red, "red"),
            (Keyword, Color::Yellow, "exit"),
            (Green, Color::Green, "green"),
            (Blue, Color::Blue, "blue"),
            (NoHighlight, Color::White, "[a-zA-Z0-9_$]+")
        );
        parser(TheLexer::lexer(string));
    }
}

fn main() {
    MyTool.start();
}
