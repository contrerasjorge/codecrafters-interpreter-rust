use std::process;

pub fn tokenizer(input: &str) {
    let mut line_number = 1;
    let mut had_error = false;
    let mut chars = input.chars().peekable();

    while let Some(char) = chars.next() {
        match char {
            '(' => println!("LEFT_PAREN ( null"),
            ')' => println!("RIGHT_PAREN ) null"),
            '{' => println!("LEFT_BRACE {{ null"),
            '}' => println!("RIGHT_BRACE }} null"),
            '*' => println!("STAR * null"),
            '.' => println!("DOT . null"),
            ',' => println!("COMMA , null"),
            '+' => println!("PLUS + null"),
            '-' => println!("MINUS - null"),
            ';' => println!("SEMICOLON ; null"),
            '=' => {
                if chars.peek() == Some(&'=') {
                    chars.next(); // consume the second '='
                    println!("EQUAL_EQUAL == null");
                } else {
                    println!("EQUAL = null");
                }
            }
            '\n' => line_number += 1,
            _ => {
                eprintln!(
                    "[line {}] Error: Unexpected character: {}",
                    line_number, char
                );
                had_error = true;
            }
        }
    }
    println!("EOF  null");

    if had_error {
        process::exit(65);
    }
}
