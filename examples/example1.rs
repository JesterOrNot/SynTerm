use std::collections::HashMap;
use synterm::CommandLineTool;

struct MyTool;

impl CommandLineTool for MyTool {
    fn evaluator_function(line: &String) -> String {
        format!("Line: {}", line)
    }
}

fn main() {
    let command_line = MyTool;
    command_line.start();
}
