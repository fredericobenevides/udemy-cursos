// mod inventory;
// mod orders;

// use fake::{Fake, Faker};

// use inventory::products::{Item, ProductCategory};
// use inventory::products;
// use inventory::products::{self, Item, ProductCategory};
// use inventory::{talk_to_manager, FLOOR_SPACE, MANAGER as INVENTORY_MANAGER};

// use inventory::products::{self};
// use inventory::{
//     talk_to_manager, Item, ProductCategory, FLOOR_SPACE, MANAGER as INVENTORY_MANAGER,
// };
// use orders::MANAGER as ORDERS_MANAGER;

// use warehouse::inventory::products::{self};
// use warehouse::inventory::talk_to_manager;
// use warehouse::{Item, ProductCategory, FLOOR_SPACE, INVENTORY_MANAGER, ORDERS_MANAGER};

// mod inventory {
//     const FLOOR_SPACE: i32 = 10000;
//     pub const MANAGER: &str = "Ivan Inventory";

//     #[derive(Debug)]
//     enum ProductCategory {
//         Ladder,
//         Hammer,
//     }

//     #[derive(Debug)]
//     struct Item {
//         name: String,
//         category: ProductCategory,
//         quantity: u32,
//     }

//     fn talk_to_manager() {
//         println!("Hey, {MANAGER}, how's your coffee?");
//     }
// }

// mod orders {
//     pub const MANAGER: &str = "Oliver Orderson";
// }

/// Create a fictional product category
fn main() {
    // println!(
    //     "Our managers are {} and {}. We have {} square feet of floor space",
    //     inventory::MANAGER,
    //     orders::MANAGER,
    //     FLOOR_SPACE
    // );

    // println!(
    //     "Our managers are {} and {}. We have {} square feet of floor space",
    //     INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    // );

    // talk_to_manager();

    // let favorite_category = inventory::products::ProductCategory::Hammer;
    // let favorite_category = ProductCategory::Hammer;
    // let favorite_category = products::ProductCategory::Hammer;
    // println!("My favorite category of item is {favorite_category:?}");

    // let tall_ladder =dd inventory::products::Item {
    // let tall_ladder = products::Item {
    //     name: String::from("Ladder-o-matic 2000"),
    //     category: favorite_category,
    //     quantity: 100,
    // };

    // let tall_ladder = Item::new(String::from("Ladder-o-matic 2000"), favorite_category, 100);
    // println!("{:#?}", tall_ladder);

    // let fake_item: Item = Faker.fake();
    // println!("{:?}", fake_item);

    // let random_category: ProductCategory = Faker.fake();
    // println!("{:?}", random_category);

    println!("This is our primary program");
}
