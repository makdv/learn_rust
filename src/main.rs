mod array;
mod data_types;
mod enums;
mod operators;
mod ownership;
mod string;
mod structs;
mod traits;
mod variables;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = &args[1];
    match cmd.as_str() {
        "data_types" => data_types::run(),
        "variables" => variables::run(),
        "ownership" => ownership::run(),
        "string" => string::run(),
        "array" => array::run(),
        "structs" => structs::run(),
        "enums" => enums::run(),
        "operators" => operators::run(),
        "traits" => traits::run(),
        _ => println!("Command not recognized"),
    }
}
