pub fn tokenizer(input: &str) {
    for char in input.chars() {
        match char {
            '(' => println!("LEFT_PAREN ( null"),
            ')' => println!("RIGHT_PAREN ) null"),
            _ => {}
        }
    }
    println!("EOF  null");
}
