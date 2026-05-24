use std::io;

pub struct InputHandler;

pub enum InputResult {
    Number(i64),
    Invalid,
    Eof,
}

impl InputHandler {
    pub fn get_number() -> InputResult {
        let mut input = String::new();
        println!("Please input your number: \n");

        match io::stdin().read_line(&mut input) {
            Ok(0) => return InputResult::Eof,
            Ok(_) => {}
            Err(_) => {
                println!("Failed to read line");
                return InputResult::Invalid;
            }
        }

        match input.trim().parse() {
            Ok(num) => InputResult::Number(num),
            Err(_) => {
                println!("Invalid input, please input a valid number\n");
                InputResult::Invalid
            }
        }
    }
}
