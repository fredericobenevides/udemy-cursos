use std::collections::HashMap;

fn main() {
    let mut sauces_to_meals = HashMap::from([
        ("Keytchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
    ]);
    // println!("{:?}", sauces_to_meals);

    sauces_to_meals.insert("Mustard", vec!["Hot dog", "Burgers", "Pretzels"]);
    // println!("{:?}", sauces_to_meals);

    println!("{:?}", sauces_to_meals.remove("Mayonnaise").unwrap());

    match sauces_to_meals.get("Mustard") {
        Some(meals) => println!("The meals were {:?}", meals),
        None => println!("There were no meals for that sauce! Oh no"),
    }

    sauces_to_meals
        .entry("Soy Sauce")
        .or_insert(vec!["Sushi", "Dumplings"]);

    println!("{:#?}", sauces_to_meals);
}
