fn color_to_number(color: &str) -> i8 {
    if color == "red" {
        1
    } else if color == "green" {
        2
    } else if color == "blue" {
        3
    } else {
        0
    }
}

fn color_to_number2(color: &str) -> i8 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial(value: i32) -> i32 {
    let mut product = 1;
    let mut count = value;

    while count > 0 {
        product *= count;
        count -= 1;
    }
    product
}

fn factorial2(value: i32) -> i32 {
    if value == 1 {
        value
    } else {
        value * factorial2(value - 1)
    }
}

fn main() {
    println!("{}", color_to_number("red"));
    println!("{}", color_to_number("green"));
    println!("{}", color_to_number("blue"));
    println!("{}", color_to_number("purple"));

    println!("{}", color_to_number2("red"));
    println!("{}", color_to_number2("green"));
    println!("{}", color_to_number2("blue"));
    println!("{}", color_to_number2("purple"));

    println!("{}", factorial(5));
    println!("{}", factorial(4));

    println!("{}", factorial2(5));
    println!("{}", factorial2(4));
}
