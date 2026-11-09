use std::io::{self, Read};

fn make_money(text: &mut String) {
    text.push_str("$$$");
}

fn trim_and_capitalize(text: &str) -> String {
    text.trim().to_uppercase()
}

fn elements(text: &str) -> Vec<&str> {
    text.split("!").collect::<Vec<&str>>()
}

fn get_identity() -> String {
    let mut first_name = String::new();
    println!("What is your first name?");
    io::stdin()
        .read_line(&mut first_name)
        .expect("Failed to collect first name");

    let mut last_name = String::new();
    println!("What is your first name?");
    io::stdin()
        .read_line(&mut last_name)
        .expect("Failed to collect last name");

    format!("{} {}", first_name.trim(), last_name.trim())
}

fn main() {
    let mut amount = String::from("1.000");
    make_money(&mut amount);
    println!("{}", amount);

    let text = trim_and_capitalize("     my text   ");
    println!("{}", text);

    let collection = "Gold!Silver!Platinum";
    println!("{:#?}", elements(collection));

    let name = get_identity();
    println!("{name}");

    let airline = "United Airlines Flight";
    let segments: Vec<&str> = airline.split(" ").collect();
}
