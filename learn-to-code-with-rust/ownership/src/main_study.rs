fn study() {
    let oranges = String::from("Oranges");
    print_my_value(oranges);
    // println!("{} is still valid", oranges);

    let burguer = String::from("Burger");
    add_fries(burguer);

    let cake = bake_cake();
    println!("I now have a {cake} cake");

    let mut current_meal = String::new();
    current_meal = add_flour(current_meal);
    show_my_meal(&current_meal);

    let mut car = String::from("Red");
    let ref1 = &mutcar;
    let ref2 = &car;
    println!("{ref1} and {ref2} and {}", &car);
    println!("{ref2}");

    let registrations = [true, false, true];
    let first = registrations[0];
    println!("{} and {:?}", first, registrations);

    let languages = [String::from("Rust"), String::from("Javascript")];
    let first = &languages[0];
    println!("{} and {:?}", first, languages);
}

// meal: String
// mut meal: String
// meal: &String
// meal: &mut String

fn print_my_value(value: String) {
    println!("Your value is {value}");
}

fn add_fries(mut meal: String) {
    meal.push_str(" and Fries");
    println!("{}", meal);
}

fn bake_cake() -> String {
    String::from("Chocolate Mousse")
}

fn add_flour(mut meal: String) -> String {
    meal.push_str("Add flour");
    meal
}

fn show_my_meal(meal: &String) {
    println!("Meal steps: {meal}");
}

fn create_city() -> &String {
    String::from("New York");
}
