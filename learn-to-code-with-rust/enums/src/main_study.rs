use std::slice::Windows;

#[derive(Debug)]
enum Beans {
    Pinto,
    Black,
}

#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}

#[derive(Debug)]
enum RestaurantItem {
    Burrito { meat: Meat, beans: Beans },
    Bowl { meat: Meat, beans: Beans },
    VeganPlate,
}

enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}

enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => {
                println!("Running the laundry with cold temperature")
            }
            LaundryCycle::Hot { temperature } => {
                println!("Running the laundry with a temperature at {temperature}");
            }
            LaundryCycle::Delicate(fabric_type) => {
                println!("Running the laundry with a delicate cycle for {fabric_type}");
            }
        }
    }
}

#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            // OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed => {
            //     println!("You item is being prepped for shipment");
            // }
            OnlineOrderStatus::Delivered => {
                println!("You item has been delivered");
            }
            // _ => {
            //     println!("Your item is not there yet");
            // }
            other_status => {
                println!("Your item is {other_status:?}");
            }
        }
    }
}

enum Milk {
    Lowfat(i32),
    Whole,
    NonDairy { kind: String },
}

impl Milk {
    fn drink(self) {
        match self {
            Milk::Lowfat(2) => {
                println!("Delicious, 2% milk is my favorite!");
            }
            Milk::Lowfat(percent) => {
                println!("You've got the lowfat {percent} percent version!");
            }
            Milk::Whole => {
                println!("You've got the whole milk!");
            }
            Milk::NonDairy { kind } => {
                println!("You've got the NonDairy {kind} milk!");
            }
        }
    }
}

fn study() {
    let lunch = RestaurantItem::Burrito {
        meat: Meat::Steak,
        beans: Beans::Pinto,
    };
    let dinner = RestaurantItem::Bowl {
        meat: Meat::Chicken,
        beans: Beans::Black,
    };
    let abandoned_meal = RestaurantItem::VeganPlate;
    println!("Lunch was {lunch:?} and dinner was {dinner:?}");
    println!("Nobody ate {abandoned_meal:?}");

    let my_computer = OperatingSystem::MacOS;
    let age = years_since_release(my_computer);
    println!("My computer's operating system is {age} years old");

    let dads_computer = OperatingSystem::Windows;
    let age = years_since_release(dads_computer);
    println!("My dad's computer is {age} years old");

    wash_laundry(LaundryCycle::Cold);
    wash_laundry(LaundryCycle::Hot { temperature: 100 });
    wash_laundry(LaundryCycle::Delicate(String::from("Silk")));

    LaundryCycle::Cold.wash_laundry();

    let hot_cycle = LaundryCycle::Hot { temperature: (100) };
    hot_cycle.wash_laundry();

    let delicate_cycle = LaundryCycle::Delicate(String::from("Silk"));
    delicate_cycle.wash_laundry();

    OnlineOrderStatus::Delivered.check();
    OnlineOrderStatus::Shipped.check();

    Milk::Lowfat(1).drink();
    Milk::Lowfat(2).drink();
    Milk::Whole.drink();

    // let my_beverage = Milk::Whole;
    let my_beverage = Milk::Lowfat(2);

    if let Milk::Lowfat(percent) = my_beverage {
        println!("Your beverage is {percent}% milk");
    }

    let my_beverage = Milk::NonDairy {
        kind: String::from("Oat"),
    };
    if let Milk::NonDairy { kind } = my_beverage {
        println!("Your beverage is {kind}% milk");
    };

    let my_beverage = Milk::Whole;

    let Milk::Lowfat(percent) = my_beverage else {
        println!("You do not have the lowfat milk!");
        return;
    };
}

fn years_since_release(os: OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => {
            println!("Quite an old operating system!");
            39
        }
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => 34,
    }
}

fn wash_laundry(cycle: LaundryCycle) {
    match cycle {
        LaundryCycle::Cold => {
            println!("Running the laundry with cold temperature")
        }
        LaundryCycle::Hot { temperature } => {
            println!("Running the laundry with a temperature at {temperature}");
        }
        LaundryCycle::Delicate(fabric_type) => {
            println!("Running the laundry with a delicate cycle for {fabric_type}");
        }
    }
}
