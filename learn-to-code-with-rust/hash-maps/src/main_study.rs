use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut menu: HashMap<String, f64> = HashMap::new();
    menu.insert(String::from("Steak"), 29.99);
    menu.insert(String::from("Tuna"), 29.99);
    menu.insert(String::from("Burger"), 14.99);

    println!("{:?}", menu);

    let mut country_capitals: HashMap<&str, &str> = HashMap::new();
    // let mut country_capitals = HashMap::<&str, &str>::new();
    country_capitals.insert("France", "Paris");
    country_capitals.insert("Germany", "Berlin");

    println!("{:?}", country_capitals);

    let data = [("Bobby", 7), ("Grant", 4), ("Ben", 6)];

    let mut years_at_company = HashMap::from(data);
    println!("{:?}", years_at_company);

    let ben = years_at_company.remove("Ben");
    println!("{:?}", ben);
    println!("{}", ben.unwrap());
    println!("{:?}", years_at_company);

    let ben = years_at_company.remove("Ben");
    println!("{:?}", ben);

    let mut coffee_pairing: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("latte");
    let milk = String::from("Oat Milk");
    coffee_pairing.insert(&drink, &milk);
    coffee_pairing.insert("Flat White", "Almond Milk");
    coffee_pairing.insert("Latte", "Pistachio Milk");
    // coffee_pairing.insert(drink.clone(), milk.clone());

    coffee_pairing.entry("Latte").or_insert("Pistacho Milk");

    coffee_pairing
        .entry("Cappuccino")
        .or_insert("Pistacho Milk");

    println!("{}", coffee_pairing.len());
    println!("{:?}", coffee_pairing);
    println!("{} {}", drink, milk);

    // let value = coffee_pairing["Flat White"];
    // let value = coffee_pairing["Cappuccino"];
    let value = coffee_pairing
        .get("Flat White")
        .copied()
        .unwrap_or("Unknown Milk");
    println!("{}", value);

    let mut concert_queue: HashSet<&str> = HashSet::new();
    concert_queue.insert("Molly");
    concert_queue.insert("Megan");
    println!("{:?}", concert_queue);

    concert_queue.insert("Molly");
    println!("{:?}", concert_queue);

    println!("{}", concert_queue.remove("Megan"));
    println!("{}", concert_queue.remove("Franny"));

    println!("{}", concert_queue.contains("Molly"));
    println!("{}", concert_queue.contains("Fred"));

    println!("{:?}", concert_queue.get("Molly"));
    println!("{:?}", concert_queue.get("Joe"));

    let mut concert_queue: HashSet<&str> = HashSet::new();
    let mut movie_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Boris");
    concert_queue.insert("Melissa");

    movie_queue.insert("Boris");
    movie_queue.insert("Phil");

    println!("{:?}", concert_queue.union(&movie_queue));
    println!("{:?}", movie_queue.union(&concert_queue));

    println!("{:?}", concert_queue.difference(&movie_queue));
    println!("{:?}", movie_queue.difference(&concert_queue));

    println!("{:?}", concert_queue.symmetric_difference(&movie_queue));
    println!("{:?}", movie_queue.symmetric_difference(&concert_queue));

    println!("{:?}", concert_queue.is_disjoint(&movie_queue));
    println!("{:?}", movie_queue.is_disjoint(&concert_queue));

    let mut attendes = HashSet::new();
    attendes.insert("Boris");
    println!("{:?}", attendes.is_subset(&movie_queue));
    println!("{:?}", attendes.is_subset(&concert_queue));

    println!("{:?}", movie_queue.is_superset(&movie_queue));
    println!("{:?}", concert_queue.is_superset(&concert_queue));
}
