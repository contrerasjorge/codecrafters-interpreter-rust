use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use tokenizer::tokenizer;

mod tokenizer;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() < 3 {
//         writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
//         return;
//     }

//     let command = &args[1];
//     let filename = &args[2];

//     match command.as_str() {
//         "tokenize" => tokenizer(filename),
//         _ => {
//             writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
//             return;
//         }
//     }
// }

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(format!("Usage: {} tokenize <filename>", args[0]));
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => tokenize(filename),
        _ => Err(format!("Unknown command: {}", command)),
    }
}

fn tokenize(filename: &str) -> Result<(), String> {
    let file_contents = fs::read_to_string(filename)
        .map_err(|e| format!("Failed to read file {}: {}", filename, e))?;

    match tokenizer::tokenizer(&file_contents) {
        Ok(_) => Ok(()),
        Err(errors) => {
            for (line, char) in errors {
                eprintln!("[line {}] Error: Unexpected character: {}", line, char);
            }
            process::exit(65);
        }
    }
}
