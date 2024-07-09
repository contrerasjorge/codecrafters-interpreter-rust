pub fn tokenizer(input: &str) -> Result<(), Vec<(usize, String)>> {
    let mut line_number = 1;
    let mut chars = input.chars().peekable();
    let mut errors = Vec::new();

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
            '!' => {
                if chars.peek() == Some(&'=') {
                    chars.next(); // consume the '='
                    println!("BANG_EQUAL != null");
                } else {
                    println!("BANG ! null");
                }
            }
            '<' => {
                if chars.peek() == Some(&'=') {
                    chars.next(); // consume the '='
                    println!("LESS_EQUAL <= null");
                } else {
                    println!("LESS < null");
                }
            }
            '>' => {
                if chars.peek() == Some(&'=') {
                    chars.next(); // consume the '='
                    println!("GREATER_EQUAL >= null");
                } else {
                    println!("GREATER > null");
                }
            }
            '/' => {
                if chars.peek() == Some(&'/') {
                    // This is a comment, consume the rest of the line
                    while let Some(next_char) = chars.next() {
                        if next_char == '\n' {
                            line_number += 1;
                            break;
                        }
                    }
                } else {
                    println!("SLASH / null");
                }
            }
            '"' => {
                let start_line = line_number;
                let mut string_content = String::new();
                let mut terminated = false;

                while let Some(next_char) = chars.next() {
                    if next_char == '"' {
                        terminated = true;
                        break;
                    } else if next_char == '\n' {
                        line_number += 1;
                    }
                    string_content.push(next_char);
                }

                if terminated {
                    println!("STRING \"{}\" {}", string_content, string_content);
                } else {
                    errors.push((start_line, "Unterminated string.".to_string()));
                }
            }

            '0'..='9' => {
                let mut number = String::from(char);
                let mut has_decimal = false;

                while let Some(&next_char) = chars.peek() {
                    if next_char.is_digit(10) {
                        number.push(chars.next().unwrap());
                    } else if next_char == '.' && !has_decimal {
                        let mut temp_chars = chars.clone();
                        temp_chars.next(); // Skip the '.'
                        if temp_chars.peek().map_or(false, |&c| c.is_digit(10)) {
                            number.push(chars.next().unwrap()); // Add the '.'
                            has_decimal = true;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                if has_decimal {
                    println!("NUMBER {} {}", number, number);
                } else {
                    println!("NUMBER {} {}.0", number, number);
                }
            }

            '\n' => line_number += 1,
            ' ' | '\r' | '\t' => {} // Ignore whitespace
            _ => errors.push((line_number, format!("Unexpected character: {}", char))),
        }
    }
    println!("EOF  null");

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
