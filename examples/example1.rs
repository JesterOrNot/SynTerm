use std::process::exit;
use synterm::{gen_lexer, gen_parse, syntax_highlight_gen, Color, CommandLineTool};

struct MyTool;

impl CommandLineTool for MyTool {
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
            (Foo, Color::Red, "foo"),
            (Bar, Color::Green, "bar"),
            (Baz, Color::Blue, "baz")
        );
        parser(TheLexer::lexer(string));
    }
}

fn main() {
    MyTool.start();
}
