use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use termion::is_tty;
use std::{
    fmt,
    fs::{File, OpenOptions},
    io::{stdout, BufRead, BufReader, Write, stdin},
    path::Path,
    process::exit,
};

/// A wrapper around ANSI codes
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Red => write!(f, "31"),
            Self::Green => write!(f, "32"),
            Self::Yellow => write!(f, "33"),
            Self::Blue => write!(f, "34"),
            Self::Magenta => write!(f, "35"),
            Self::Cyan => write!(f, "36"),
        }
    }
}

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

#[macro_export]
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
                #[regex = $target]
                $token,
            )*
        }
    };
    ($enumName:ident) => {
        #[derive(Logos, Debug, Clone, PartialEq, Eq)]
        enum $enumName {
            #[end]
            End,
            #[error]
            Error,
            #[token = " "]
            Whitespace
        }
    };
}

#[macro_export]
macro_rules! gen_parse {
    ($enumName:ident, $funcName:ident, $(($token:ident, $ansi:expr)), *) => {
        use logos::{Logos, Lexer};
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
    ($enumName:ident, $funcName:ident) => {
        use logos::{Logos, Lexer};
        fn $funcName(mut tokens: Lexer<$enumName, &str>) {
            while tokens.token != $enumName::End {
                match tokens.token {
                    _ => print!("{}", tokens.slice())
                }
                tokens.advance();
            }
        }
    }
}

#[macro_export]
macro_rules! syntax_highlight_gen {
    ($enumName:ident, $funcName:ident, $(($token:ident, $ansi:expr, $target:literal)), *) => {
        use synterm::{gen_lexer, gen_parse};
        gen_lexer!($enumName, $(($token, $target)),*);
        gen_parse!($enumName, $funcName, $(($token, $ansi)),*);
    };
}

#[allow(dead_code)]
/// This Trait is how you make your command line tool it is the center of all synterm programs
pub trait CommandLineTool {
    /// The input prompt defaults to `>>> `
    const PROMPT: &'static str = ">>> ";
    /// Path to the history file defaults to `/tmp/history.txt`
    const HISTORY_FILE_PATH: &'static str = "/tmp/history.txt";
    /// Do not implement! This is used internally
    fn get_hist(n: usize) -> String {
        match lines_from_file(Self::HISTORY_FILE_PATH).nth(n) {
            Some(n) => n,
            None => "".to_string(),
        }
    }

    fn start(&self) {
        if is_tty(&File::open("/dev/stdin").unwrap()) {
            self.repl();
            exit(0)
        }
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        print!("{}", Self::evaluator_function(&buffer));
    }

    /// Starts the REPL
    fn repl(&self) {
        let mut cursor_position = 0;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(Self::HISTORY_FILE_PATH)
            .unwrap();
        let mut position = lines_from_file(Self::HISTORY_FILE_PATH).count();
        let mut buffer = String::new();
        loop {
            enable_raw_mode().unwrap();
            // Move to the left, clear line, print prompt
            print!("\x1b[1000D\x1b[0K{}\x1b[m", Self::PROMPT);
            // Print buffer
            Self::syntax_highlight(&buffer);
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
                            if position > 0 {
                                position -= 1;
                            } else {
                                position = lines_from_file(Self::HISTORY_FILE_PATH).count();
                            }
                            print!("\x1b[1000D\x1b[0K{}", Self::PROMPT);
                            buffer = Self::get_hist(position);
                            print!("\x1b[1000D");
                            cursor_position = buffer.len();
                        }
                        KeyCode::Down => {
                            if position < lines_from_file(Self::HISTORY_FILE_PATH).count() {
                                position += 1;
                            } else {
                                position = 0;
                            }
                            buffer = Self::get_hist(position);
                            cursor_position = buffer.len();
                        }
                        KeyCode::Enter => match buffer.as_str() {
                            "" => {
                                println!("\r");
                                position = lines_from_file(Self::HISTORY_FILE_PATH).count();
                            }
                            _ => {
                                println!("\r");
                                file.write_all(format!("{}\n", buffer).as_bytes()).unwrap();
                                position = lines_from_file(Self::HISTORY_FILE_PATH).count();
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
    /// This drives the syntax highlighting it should consist of one macro and function call
    /// <br>
    /// First the macro cll is `syntax_highlight_gen!` this will generate the lexers this will take the following paramaters
    /// 1. enumName this will be the name of the enum that will serve as our tokens put an identifier here that hasn't been used i.e. `gen_lexer!(TheLexer)`
    /// 2. funcName -- put in the name of your parser method
    /// 3. args this is as many as you want and will actually define new tokens, for each pattern of creating tokens you want add the following pair (Identifier, Color, Regex) i.e. `(Number, Color::Red, r"[0-9]+")`  (we get the Color enum from `syntem::Color`) a full example might look like this
    /// ```rust
    /// use synterm::{syntax_highlight_gen, Color};
    /// syntax_highlight_gen!(TheLexer, parser, (Foo, Color::Red, "foo"), (Bar, Color::Green, "bar"));
    /// ```
    /// Now for the function call the parse function we create with `syntax_highlight_gen!` in which one calls
    /// <br>
    /// ParserName(TokenNames::lexer(string));
    /// from the last 2 snippets it is `parser(TheLexer::lexer(string));`
    ///
    /// <br>
    ///
    /// Lets put this together
    ///
    /// ```rust
    /// use synterm::{syntax_highlight_gen, Color};
    /// fn syntax_highlight(string: &str) {
    ///     syntax_highlight_gen!(TheLexer, parser, (Foo, Color::Red, "foo"), (Bar, Color::Green, "bar"));
    ///     parser(TheLexer::lexer(string));
    /// }
    /// ```
    fn syntax_highlight(string: &str) {
        gen_lexer!(TheLexer);
        gen_parse!(TheLexer, parse);
        parse(TheLexer::lexer(string));
    }
    /// This should take a line and return the evaluated output after evaluation
    fn evaluator_function(line: &String) -> String;
}
