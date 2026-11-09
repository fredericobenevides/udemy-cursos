#[derive(Debug)]
struct DeliSandwitch {}

fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }
}

impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

enum Cheesesteak<T> {
    Plain,
    Topping(T),
}

fn main() {
    println!("{}", identity::<i32>(5));
    println!("{}", identity::<i8>(5));
    println!("{}", identity::<u32>(5));
    println!("{}", identity(13.14));
    println!("{}", identity::<f64>(13.14));
    println!("{}", identity::<&str>("hello"));
    println!("{}", identity::<String>(String::from("hello")));
    println!("{}", identity::<bool>(true));
    println!("{:?}", identity::<DeliSandwitch>(DeliSandwitch {}));

    make_tuple(5, "hello");
    make_tuple(5, 13);
    make_tuple(true, 3.14);
    make_tuple(true, false);

    let gold_chest = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold",
    };
    println!("{:?}", gold_chest);
    println!("{}", gold_chest.capital_captain());

    let mut silver_chest = TreasureChest {
        captain: String::from("Bloodsaild"),
        treasure: String::from("         Silver         "),
    };
    silver_chest.clean_treasure();
    println!("{:?}", silver_chest);
    println!("{}", silver_chest.capital_captain());

    let special_chest = TreasureChest {
        captain: String::from("Bootyplunder"),
        treasure: ["Gold", "Silver", "Platinum"],
    };
    println!("{:?}", special_chest);
    println!("{}", special_chest.amount_of_treasure());
    println!("{}", special_chest.capital_captain());

    let mushroom = Cheesesteak::Topping("mushroom");
    let onions = Cheesesteak::Topping("onion");
    let topping = "bacon".to_string();
    let bacon = Cheesesteak::Topping(&topping);

    let mut plain: Cheesesteak<String> = Cheesesteak::Plain;
    // plain = Cheesesteak::Topping("sausage");
    plain = Cheesesteak::Topping("sausage".to_string());
}

fn identity<T>(value: T) -> T {
    value
}
