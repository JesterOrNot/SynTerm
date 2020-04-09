# SynTerm

[![Gitpod Ready-to-Code](https://img.shields.io/badge/Gitpod-Ready--to--Code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/JesterOrNot/synterm)

A Rust library for making beautiful REPLs and Shells with fish like as you type syntax highlighting

## Quick Start

```rust
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
```

## Getting Started

Add the following to your Cargo.toml's dependency section

```toml
synterm = "0.2.11"
logos = "0.9.7"
```

## Examples

  - [JesterOrNot/PyRepl](https://github.com/JesterOrNot/PyRepl)
  - [JesterOrNot/MathRepl](https://github.com/JesterOrNot/MathRepl)

## Contributing

See TODO.md for ways to contribute

Open it in Gitpod everything is all ready for you!

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/JesterOrNot/synterm)
