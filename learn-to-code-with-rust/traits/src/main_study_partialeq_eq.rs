#[derive(PartialEq, Eq)]
struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

fn main() {
    let a = Flight::new("New York", "London", "08:00");
    let b = Flight::new("New York", "London", "08:00");
    let c = Flight::new("New York", "London", "08:00");

    println!("{}", a == a);
    println!("{}", a == b);
    println!("{}", b == a);
    println!("{}", b == c);

    let division = 0.0 / 0.0;
    println!("{}", division);

    let value = 3.4;
    println!("{}", value == value);
    println!("{}", division == division);
}
