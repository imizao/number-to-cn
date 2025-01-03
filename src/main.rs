mod converter;
mod input_handler;

use converter::Converter;
use input_handler::InputHandler;

fn main() {
    loop {
        match InputHandler::get_number() {
            Some(number) => {
                let result = Converter::number_to_zhcn(number);
                println!("Result is: {:?} \n", result);
            }
            None => continue,
        }
    }
}