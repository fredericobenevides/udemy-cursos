#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passenger: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passenger: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passenger,
        }
    }

    fn change_destination(&mut self, new_destination: String) {
        self.destination = new_destination;
    }

    fn increase_price(&mut self) {
        self.price *= 1.2;
    }

    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
}

fn main() {
    let mut my_flight = Flight::new(String::from("Brazil"), String::from("Cancun"), 1000.0, 40);
    println!("{:?}", my_flight);
    my_flight.change_destination(String::from("Miami"));
    my_flight.increase_price();
    my_flight.itinerary();

    println!("{:?}", my_flight);

    let another_flight = Flight {
        origin: String::from("Rome"),
        destination: String::from("Paris"),
        ..my_flight
    };
    println!("{:?}", another_flight);
}
