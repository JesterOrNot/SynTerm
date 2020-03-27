use std::process::exit;
use synterm::{CommandLineTool, gen_lexer, gen_parse, Color};

struct MyTool;

impl CommandLineTool for MyTool {
    fn evaluator_function(line: &String) -> String {
        match line.as_str() {
            "exit" => {
                exit(0);
            }
            _ => {
                format!("Line: {}", line)
            }
        }
    }
    fn syntax_highlight(string: &str) {
        gen_lexer!(TheLexer, (Foo, "foo"), (Bar, "bar"));
        gen_parse!(TheLexer, parser, (Foo, Color::Red), (Bar, Color::Green));
        parser(TheLexer::lexer(string));
    }
}

fn main() {
    let command_line = MyTool;
    command_line.start();
}
