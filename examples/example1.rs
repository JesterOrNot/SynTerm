use std::collections::HashMap;
use synterm::{CommandLineTool, gen_lexer, gen_parse};

struct MyTool;

impl CommandLineTool for MyTool {
    fn evaluator_function(line: &String) -> String {
        format!("Line: {}", line)
    }
    fn syntax_highlight(string: &str) {
        gen_lexer!(TheLexer, (Foo, "foo"));
        gen_parse!(TheLexer, parser, (Foo,"31"));
        parser(TheLexer::lexer(string));
    }
}

fn main() {
    let command_line = MyTool;
    command_line.start();
}
