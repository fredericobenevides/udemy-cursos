fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is {result}");
}

fn study() {
    even_or_odd(17);
    even_or_odd(100);

    let evaluation = true;

    match evaluation {
        true => {
            println!("The value is true");
        }
        false => {
            println!("The value is false");
        }
    }

    let value = match evaluation {
        true => 20,
        false => 40,
    };
    print!("{value}");

    let season = "winter";
    match season {
        "summer" => println!("School's out"),
        "winter" => println!("Brr, so cold!"),
        _ => println!("Lots of rain!"),
    }

    let number = 0;
    match number {
        2 | 4 | 6 | 8 => println!("{number} is even"),
        1 | 3 | 5 => println!("{number} is odd"),
        _ => println!("Unknown for now"),
    }

    let number = 5;
    match number {
        value if value % 2 == 0 => println!("{value} is an even number"),
        x if x % 2 != 0 => println!("{x} is an odd number"),
        _ => unreachable!(),
    }
}
