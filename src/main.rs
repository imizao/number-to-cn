mod converter;
mod input_handler;

use converter::Converter;
use input_handler::{InputHandler, InputResult};

fn main() {
    loop {
        match InputHandler::get_number() {
            InputResult::Number(number) => {
                let result = Converter::number_to_zhcn(number);
                println!("Result is: {} \n", result);
            }
            InputResult::Invalid => continue,
            InputResult::Eof => break,
        }
    }
}
