use std::io::{self, Write};
use std::process;

/// Represents the possible argument types parsed from the CLI
#[derive(Debug)]
enum Arg {
    Int(i32),
    Double(f64),
    String(String),
}

/// Type alias for our callback functions
type CallbackFunc = fn(Vec<Arg>);

struct CallbackEntry {
    cmd: String,
    func: CallbackFunc,
}

struct Callbacks {
    entries: Vec<CallbackEntry>,
    head: String,
}

impl Callbacks {
    fn new() -> Self {
        let mut cb = Self {
            entries: Vec::new(),
            head: String::from("Menu\n"),
        };
        cb.add_callback("g", "go", go);
        cb.add_callback("q", "quit", |_| process::exit(0));
        cb
    }

    fn add_callback(&mut self, cmd: &str, name: &str, func: CallbackFunc) {
        self.entries.push(CallbackEntry {
            cmd: cmd.to_string(),
            func,
        });
        self.head.push_str(&format!("{} {}\n", cmd, name));
    }

    fn lookup(&self, cmd: &str) -> Option<CallbackFunc> {
        self.entries.iter()
            .find(|e| e.cmd == cmd)
            .map(|e| e.func)
    }

    fn header(&self) {
        print!("{}", self.head);
    }
}

struct Menu {
    callbacks: Callbacks,
}

impl Menu {
    fn new() -> Self {
        Self {
            callbacks: Callbacks::new(),
        }
    }

    fn run(&self) {
        let clear_screen = "\x1B[2J\x1B[;H";

        loop {
            print!("{}", clear_screen);
            self.callbacks.header();
            print!("<select>: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() || input.is_empty() {
                break;
            }

            let mut parts = input.split_whitespace();
            let cmd = match parts.next() {
                Some(c) => c,
                None => continue,
            };

            if let Some(cb) = self.callbacks.lookup(cmd) {
                let args = self.parse_args(parts);
                cb(args);
            }
        }
    }

    fn parse_args<'a>(&self, tokens: impl Iterator<Item = &'a str>) -> Vec<Arg> {
        tokens.map(|token| {
            if let Ok(i) = token.parse::<i32>() {
                Arg::Int(i)
            } else if let Ok(d) = token.parse::<f64>() {
                Arg::Double(d)
            } else {
                Arg::String(token.to_string())
            }
        }).collect()
    }
}

/// The "go" callback implementation
fn go(args: Vec<Arg>) {
    print!("go ");
    for arg in args {
        match arg {
            Arg::Int(i) => print!("int: {} ", i),
            Arg::Double(d) => print!("double: {} ", d),
            Arg::String(s) => print!("string: {} ", s),
        }
    }
    io::stdout().flush().unwrap();

    // Wait for user to press enter (mimics C++ dummy getline)
    let mut dummy = String::new();
    let _ = io::stdin().read_line(&mut dummy);
}

fn main() {
    let menu = Menu::new();
    menu.run();
}

