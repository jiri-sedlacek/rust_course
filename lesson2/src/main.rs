use slug::slugify;
use std::io;
use std::env;

fn main() {
    let array: [&str; 6] = ["lowercase", "uppercase", "no-spaces", "slugify", "reverse", "snakecase"];
    let mut input = String::new();
    println!("Enter text:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut input = input.trim().to_string();
    let mut trans_input = input;
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("No args given, printing original text");
        println!("{input}");
        println!("Btw: allowed args are: {:?}", array);
    } else {
        println!("Applying transmutations");
        for transformation in &args[1..] {
            if !array.contains(&transformation.as_str()) {
                println!("Invalid transformation: {}", transformation);
                continue;
            }
            // hashmap?
            trans_input = match transformation.as_str() {
                "lowercase" => trans_input.to_lowercase(),
                "uppercase" => trans_input.to_uppercase(),
                "no-spaces" => trans_input.replace(' ', ""),
                "reverse" => trans_input.chars().rev().collect(),
                "snakecase" => trans_input.split_whitespace()
                    .map(|w| w.to_lowercase())
                    .collect::<Vec<String>>()
                    .join("_"),
                "slugify" => slugify(&trans_input),

                _ => String::from("Entered option for the transmutation doesn't match any existing case."),
            };
        }
        println!("Transformed text: {}", trans_input);
    }
}
