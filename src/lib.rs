#![crate_type = "lib"]
#![crate_name = "synterm"]

use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use logos::{Lexer, Logos};
use std::collections::HashMap;
use std::env::{current_dir, var};
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{stdout, BufRead, BufReader, Write};
use std::path::Path;
use std::process::exit;

fn lines_from_file<T: AsRef<Path>>(filename: T) -> impl Iterator<Item = String> {
    let file = File::open(filename);
    let file = match file {
        Ok(n) => n,
        Err(_) => {
            println!("Error! File not found!");
            exit(0);
        }
    };
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line"))
}

macro_rules! gen_lexer {
    ($enumName:ident, $(($token:ident,$target:literal)), *) => {
        #[derive(Logos, Debug, Clone, PartialEq, Eq)]
        enum $enumName {
            #[end]
            End,
            #[error]
            Error,
            #[token = " "]
            Whitespace,
            $(
                #[token = $target]
                $token,
            )*
        }
    };
}

macro_rules! gen_parse {
    ($enumName:ident, $funcName:ident, $(($token:ident, $ansi:literal)), *) => {
        fn $funcName(mut tokens: Lexer<$enumName, &str>) {
            while tokens.token != $enumName::End {
                match tokens.token {
                    $(
                        $enumName::$token => print!("\x1b[{}m{}\x1b[m", $ansi, tokens.slice()),
                    )*
                    _ => print!("{}", tokens.slice())
                }
                tokens.advance();
            }
        }
    };
}

#[macro_export]
macro_rules! apply_args {

    ($string:ident, $(($token:ident, $ansi:literal, $vals:literal)), *) => {
        $(
            gen_lexer!(TheLexer, ($token, $vals));
            gen_parse!(TheLexer, parser, ($token, $ansi));
            parser(TheLexer::lexer(&$string))
        )*
    }
}

macro_rules! syntax_highlight {

    ($string:ident, $(($token:ident, $ansi:literal, $vals:literal)), *) => {
        $(
            apply_args!()
        )*
    }
}

#[allow(dead_code)]
/// This Trait is how you make your command line tool
pub trait CommandLineTool {
    const PROMPT: &'static str = ">>> ";
    const HISTORY_FILE_PATH: &'static str = "/tmp/history.txt";
    fn get_hist(n: usize) -> String {
        match lines_from_file("").nth(n) {
            Some(n) => n,
            None => "".to_string(),
        }
    }
    fn start(&self) {
        let mut cursor_position = 0;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(Self::HISTORY_FILE_PATH)
            .unwrap();
        let mut positon = lines_from_file(Self::HISTORY_FILE_PATH).count();
        let mut buffer = String::new();
        loop {
            // Move to the left, clear line, print prompt
            print!("\x1b[1000D\x1b[0K{}\x1b[m", Self::PROMPT);
            // Print buffer
            apply_args!(buffer, (Foo, "31", "foo"));
            // Move to the left and move to the right cursor position
            print!("\x1b[1000D\x1b[{}C", cursor_position + Self::PROMPT.len());
            stdout().flush().unwrap();
            let event = read().unwrap();
            if let Event::Key(n) = event {
                match n {
                    crossterm::event::KeyEvent {
                        code: m,
                        modifiers: z,
                    } => match m {
                        KeyCode::Char(v) => match z {
                            KeyModifiers::CONTROL => {
                                buffer.clear();
                                cursor_position = 0;
                                match v {
                                    'd' => {
                                        disable_raw_mode().unwrap();
                                        println!();
                                        exit(0);
                                    }
                                    _ => {
                                        println!("^{}", v.to_uppercase());
                                        continue;
                                    }
                                }
                            }
                            _ => {
                                buffer.insert(cursor_position, v);
                                cursor_position += 1;
                            }
                        },
                        KeyCode::Backspace => {
                            if cursor_position > 0 {
                                cursor_position -= 1;
                                buffer.remove(cursor_position);
                            }
                        }
                        KeyCode::Left => {
                            if cursor_position > 0 {
                                cursor_position -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if cursor_position < buffer.len() {
                                cursor_position += 1;
                            }
                        }
                        KeyCode::Up => {
                            if positon > 0 {
                                positon -= 1;
                            }
                            print!("\x1b[1000D\x1b[0K{}", Self::PROMPT);
                            buffer = Self::get_hist(positon);
                            print!("\x1b[1000D");
                            cursor_position = buffer.len();
                        }
                        KeyCode::Down => {
                            if positon < lines_from_file(Self::HISTORY_FILE_PATH).count() {
                                positon += 1;
                            }
                            buffer = Self::get_hist(positon);
                            cursor_position = buffer.len();
                        }
                        KeyCode::Enter => match buffer.as_str() {
                            "" => {
                                println!("\r");
                            }
                            _ => {
                                println!("\r");
                                file.write_all(format!("{}\n", buffer).as_bytes()).unwrap();
                                positon += 1;
                                disable_raw_mode().unwrap();
                                let output = Self::evaluator_function(&buffer);
                                println!("{}\r", output);
                                enable_raw_mode().unwrap();
                                print!("\r");
                                cursor_position = 0;
                                buffer.clear();
                            }
                        },
                        _ => {}
                    },
                }
            }
        }
    }
    /// This should take a line and return the evaluated output after evaluation
    fn evaluator_function(line: &String) -> String;
}
