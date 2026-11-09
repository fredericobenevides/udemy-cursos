#[derive(PartialEq)]
struct BustTrip {
    origin: String,
    destination: String,
    time: String,
}

impl BustTrip {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

impl PartialEq<Flight> for BustTrip {
    fn eq(&self, other: &Flight) -> bool {
        self.time == other.time
    }
}

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

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}

impl PartialEq<BustTrip> for Flight {
    fn eq(&self, other: &BustTrip) -> bool {
        self.time == other.time
    }
}

fn main() {
    let a = Flight::new("New York", "London", "08:00");
    let b = Flight::new("New York", "London", "23:20");
    let c = Flight::new("New York", "Los Angeles", "08:00");
    println!("{}", a == b);
    println!("{}", a.eq(&b));
    println!("{}", a.ne(&b));
    println!("{}", a == c);
    println!("{}", a != c);
    println!("");

    let a = Flight::new("New York", "London", "08:00");
    let b = BustTrip::new("New York", "London", "08:00");
    println!("{}", a == a);
    println!("{}", a == b);
    println!("{}", b == a);
    println!("{}", b == b);
}
