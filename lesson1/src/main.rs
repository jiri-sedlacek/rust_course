use std::env;
use std::env::Args;
use std::io;

fn main() {
    let mut arguments: Args = env::args();
    let program_name: Option<String> = arguments.next();
    let program_name = program_name.unwrap();
    println!("App path: {program_name}");
    println!("Enter your name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim();
    println!("Hello, {}!", name);
}
