use std::io;

pub struct InputHandler;

impl InputHandler {
    pub fn get_number() -> Option<i64> {
        let mut input = String::new();
        println!("Please input your number: \n");

        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read line");
            return None;
        }

        match input.trim().parse() {
            Ok(num) => Some(num),
            Err(_) => {
                println!("Invalid input, please input a valid number\n");
                None
            }
        }
    }
} 