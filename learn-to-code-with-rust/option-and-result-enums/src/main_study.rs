fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Option::Some(true)
    } else if item_is_in_system {
        Option::Some(false)
    } else {
        Option::None
    }
}

#[derive(Debug, Copy, Clone)]
enum MyOption {
    Some(i32),
    None,
}

impl MyOption {
    fn unwrap(self) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Uh oh"),
        }
    }

    fn unwrap_or(self, fallback_value: i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value,
        }
    }
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn operation(great_success: bool) -> Result<&'static str, &'static str> {
    if great_success {
        Ok("Sucess")
    } else {
        Err("Error")
    }
}

fn main() {
    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    // let bass: Option<&String> = musical_instruments.get(2);
    // println!("{:?}", bass);
    // // let valid_instrument = bass.unwrap();
    // let valid_instrument = bass.expect("Unable to retrieve element");
    // println!("{valid_instrument}");

    // let invalid_instrument = musical_instruments.get(100);
    // println!("{:?}", invalid_instrument);
    // // invalid_instrument.unwrap();
    // invalid_instrument.expect("Unable to retrieve musical instrument");

    let bass: Option<&String> = musical_instruments.get(2);
    play(bass);
    println!("{:?}", bass);

    let invalid_instrument = musical_instruments.get(100);
    play(invalid_instrument);

    let availability = is_item_in_stock(true, true);

    // match availability {
    //     Option::Some(true) => println!("Yes, the item is available"),
    //     Option::Some(false) => println!("No, the item is not in the stock"),
    //     Option::None => println!("Your item doesn't exist in our system"),
    // }

    match availability {
        Some(true) => println!("Yes, the item is available"),
        Some(false) => println!("No, the item is not in the stock"),
        None => println!("Your item doesn't exist in our system"),
    }

    let present_value = Some(13);
    let missing_value: Option<i32> = None;

    println!("{}", present_value.unwrap_or(0));
    println!("{}", missing_value.unwrap_or(100));

    let some_option = MyOption::Some(100);
    println!("{}", some_option.unwrap());
    println!("{}", some_option.unwrap_or(13));

    let none_option = MyOption::None;
    // println!("{}", none_option.unwrap());
    println!("{}", none_option.unwrap_or(13));

    // let ok: Result<i32, &str> = Result::Ok(5);
    let ok: Result<i32, &str> = Ok(5);
    println!("{:?}", ok);
    // let disaster: Result<i32, &str> = Result::Err("Something went wrong");
    let disaster: Result<i32, &str> = Err("Something went wrong");
    println!("{:?}", disaster);

    let text = "50";
    let text_as_number = text.parse::<i32>();
    println!("{:?}", text_as_number);

    let text = "Alabama";
    let text_as_number = text.parse::<i32>();
    println!("{:?}", text_as_number);

    let result = divide(10.0, 2.0);

    // match result {
    //     Ok(calculation) => println!("Result: {}", calculation),
    //     Err(message) => println!("Error: {}", message),
    // }

    // println!("{}", result.unwrap());
    println!("{}", result.is_ok());

    let result = divide(10.0, 0.0);
    // println!("{}", result.unwrap());
    // println!("{}", result.unwrap_or(0.0));
    println!("{}", result.is_err());

    let my_result = operation(true);
    let content = match my_result {
        Ok(message) => message,
        Err(error) => error,
    };

    println!("{}", my_result.unwrap());

    let mut sauces = vec!["Mayonaise", "Ketchup", "Ranch"];

    // if let Some(sauce) = sauces.pop() {
    //     println!("The next sauce is {sauce}");
    // }

    while let Some(sauce) = sauces.pop() {
        println!("The next sauce is {sauce}");
    }
}

fn play(instrument_option: Option<&String>) {
    match instrument_option {
        Option::Some(instrument) => println!("Playing the {instrument}"),
        Option::None => println!("Singing with my voice"),
    }
}
