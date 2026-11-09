fn main() {
    let is_concert = true;
    let is_event = is_concert;
    println!("is_concert: {}", is_concert);
    println!("is_event: {}", is_event);

    let sushi = "Salmon";
    let dinner = sushi;
    println!("sushi: {}", sushi);
    println!("dinner: {}", dinner);

    let sushi2 = String::from("Salmon");
    println!("sushi2: {}", sushi2);

    let dinner2 = sushi2;
    // println!("sushi2: {}", sushi2);
    println!("dinner2: {}", dinner2);

    let mut meal = String::from("Salmon");
    meal = eat_meal(meal);
    println!("meal: {}", meal);

    let mut trip = start_trip();
    visit_philadelphia(&mut trip);
    trip.push_str(" and ");
    visit_new_york(&mut trip);
    trip.push_str(" and ");
    visit_boston(&mut trip);
    show_itinerary(&trip);
}

fn clear(mut value: String) -> String {
    value.clear();
    value
}

fn eat_meal(meal: String) -> String {
    clear(meal)
}

fn start_trip() -> String {
    String::from("The plan is...")
}

fn visit_philadelphia(trip: &mut String) {
    trip.push_str("Philadelphia");
}

fn visit_new_york(trip: &mut String) {
    trip.push_str("New York");
}

fn visit_boston(trip: &mut String) {
    trip.push_str("Boston");
}

fn show_itinerary(trip: &String) {
    println!("{}.", trip);
}
