use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::{self, exit};

mod error;
mod expr;
mod lox_object;
mod parser;
mod scanner;
mod token;
mod token_type;

fn run_file(path: &str) -> io::Result<()> {
    let mut f = match File::open(path) {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };

    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;
    match run(buffer) {
        error::RuntimeResult::Safe => {}
        error::RuntimeResult::LexicalError => exit(65),
        error::RuntimeResult::ParserError => exit(65),
    };
    Ok(())
}

fn run_prompt() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                match run(line) {
                    error::RuntimeResult::Safe => {}
                    error::RuntimeResult::LexicalError => {}
                    error::RuntimeResult::ParserError => {}
                };
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

fn run(source: String) -> error::RuntimeResult {
    let mut scanner: scanner::Scanner = scanner::Scanner::new(source);
    let tokens = match scanner.scan_tokens() {
        Ok(o) => o,
        Err(e) => {
            e.iter().for_each(|error| error.report());
            return error::RuntimeResult::LexicalError;
        }
    };
    let mut parser: parser::Parser = parser::Parser::new(tokens);
    let _expr = match parser.parse() {
        Ok(o) => println!("{}", o.display()),
        Err(e) => {
            e.report();
            return error::RuntimeResult::ParserError;
        }
    };
    error::RuntimeResult::Safe
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rloxj [script]");
        process::exit(7);
    } else if args.len() == 2 {
        match run_file(args[1].as_str()) {
            Ok(()) => (),
            Err(error) => panic!("There was a problem opening the file: {:?}", error),
        }
    } else {
        match run_prompt() {
            Ok(()) => (),
            Err(error) => panic!("There was a problem opening the file: {:?}", error),
        }
    }
}
