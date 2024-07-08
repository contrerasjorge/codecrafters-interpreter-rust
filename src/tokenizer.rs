pub fn tokenizer(input: &str) {
    for char in input.chars() {
        match char {
            '(' => println!("LEFT_PAREN ( null"),
            ')' => println!("RIGHT_PAREN ) null"),
            '{' => println!("LEFT_BRACE {{ null"),
            '}' => println!("RIGHT_BRACE }} null"),
            _ => {}
        }
    }
    println!("EOF  null");
}
