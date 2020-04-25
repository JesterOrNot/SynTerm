use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
    tty::IsTty,
};
use std::{
    fmt,
    fs::{File, OpenOptions},
    io::{stdin, stdout, BufRead, BufReader, Write},
    path::Path,
    process::exit,
};

/// A wrapper around ANSI escape sequences
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    White,
    Cyan,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::White => write!(f, "26"),
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

pub fn calculate_whitespace(string: &str) -> usize {
    let mut result = 0;
    let string = string.to_owned().into_bytes();
    let mut it = string.iter().peekable();
    while let Some(&c) = &it.peek() {
        match c {
            b'\x01' => {
                while it.peek().is_some() && **it.peek().unwrap() != b'\x02' {
                    it.next();
                }
                it.next();
            }
            _ => {
                result += 1;
                it.next();
            }
        }
    }
    result
}

#[macro_export]
macro_rules! gen_lexer {
    ($enumName:ident, $(($token:ident,$target:literal)), *) => {
        #[derive(Logos, Debug, Clone, PartialEq, Eq)]
        enum $enumName {
            #[error]
            Error,
            #[token(" ")]
            Whitespace,
            $(
                #[regex($target)]
                $token,
            )*
        }
    };
    ($enumName:ident) => {
        #[derive(Logos, Debug, Clone, PartialEq, Eq)]
        enum $enumName {
            #[error]
            Error,
            #[token(" ")]
            Whitespace
        }
    };
}

#[macro_export]
macro_rules! gen_parse {
    ($enumName:ident, $funcName:ident, $(($token:ident, $ansi:expr)), *) => {
        use logos::{Logos, Lexer};
        let mut CSI = "";
        if cfg!(not(target_os = "linux")) {
            CSI = "$([char]27)";
        } else {
            CSI = r"\x1b";
        }
        fn $funcName(mut tokens: Lexer<$enumName>) {
            let mut CSI = "";
            if cfg!(target_os = "windows") {
                CSI = "$([char]27)";
            } else {
                CSI = r"\x1b";
            }
            while let Some(token) = tokens.next() {
                match token {
                    $(
                        $enumName::$token => print!("{}[{}m{}{}[m", CSI, $ansi, tokens.slice(), CSI),
                    )*
                    _ => print!("{}", tokens.slice())
                }
            }
        }
    };
    ($enumName:ident, $funcName:ident) => {
        use logos::{Logos, Lexer};
        fn $funcName(mut tokens: Lexer<$enumName>) {
            while let Some(token) = tokens.next() {
                match token {
                    _ => print!("{}", tokens.slice())
                }
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
        if stdin().is_tty() {
            self.repl();
            exit(0)
        }
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        print!("{}", Self::evaluator_function(&buffer));
    }

    /// Starts the REPL
    fn repl(&self) {
        let mut CSI = "";
        if cfg!(target_os = "windows") {
            CSI = "$([char]27)";
        } else {
            CSI = r"\x1b";
        }
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
            print!("{}[1000D{}[0K{}{}[m", CSI, CSI, Self::PROMPT, CSI);
            // Print buffer
            Self::syntax_highlight(&buffer);
            // Move to the left and move to the right cursor position
            print!(
                "{}[1000D{}[{}C",
                CSI,
                CSI,
                cursor_position + calculate_whitespace(Self::PROMPT)
            );
            stdout().flush().unwrap();
            let event = read().unwrap();
            if let Event::Key(n) = event {
                match n {
                    crossterm::event::KeyEvent {
                        code: m,
                        modifiers: z,
                    } => match m {
                        KeyCode::Char(v) => match z {
                            KeyModifiers::CONTROL => match v {
                                'd' => {
                                    disable_raw_mode().unwrap();
                                    println!();
                                    exit(0);
                                }
                                _ => {}
                            },
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
                            print!("{}[1000D{}[0K{}", CSI, CSI, Self::PROMPT);
                            buffer = Self::get_hist(position);
                            print!("{}[1000D", CSI);
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
                                println!("{}", output);
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
