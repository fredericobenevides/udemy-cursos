fn main() {
    // let mut pizza_diameters: Vec<i32> = vec![8, 10, 12, 14];
    // pizza_diameters.push(16);
    // pizza_diameters.push(18);

    // pizza_diameters.insert(0, 4);

    // let last_pizza_diameter = pizza_diameters.pop();
    // println!("{:?}", last_pizza_diameter);

    // let third_diameter_from_start = pizza_diameters.remove(2);
    // println!("{:}", third_diameter_from_start);

    // println!("{:?}", pizza_diameters);

    let pizza_diameters: Vec<i32> = vec![8, 10, 12, 14];

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_toppings = vec![pepperoni, mushroom, sausage];

    let value = &pizza_diameters[2];
    println!("{value}");

    let value = &pizza_toppings[2];
    println!("{value}");

    // let value = pizza_diameters[100];
    // println!("{value}");

    // let value = &pizza_diameters[100];
    // println!("{value}");

    let pizza_slice = &pizza_diameters[1..];
    println!("{pizza_slice:?}");

    // let option = pizza_diameters.get(2);
    let option = pizza_diameters.get(50);
    println!("{:?}", option);
    match option {
        Some(topping) => println!("The topping is {topping}"),
        None => println!("No value at the index position"),
    }

    let mut delicious_toppings = pizza_toppings;
    let topping_reference = &delicious_toppings[1];
    println!("The topping is {topping_reference}");

    delicious_toppings.push(String::from("Olives"));

    // println!("The topping is {topping_reference}");

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let mut pizza_toppings = vec![pepperoni, mushroom, sausage];
    pizza_toppings[1] = String::from("Olives");
    println!("{pizza_toppings:#?}");

    let target_topping = &mut pizza_toppings[2];
    target_topping.push_str(" and Meatballs");
    println!("{pizza_toppings:#?}");

    let mut seasons: Vec<&str> = Vec::with_capacity(4);
    println!(
        "Length: {}. Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );

    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");
    seasons.push("Spring");

    println!(
        "Length: {}. Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );

    seasons.push("Summer");

    println!(
        "Length: {}. Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );
}
