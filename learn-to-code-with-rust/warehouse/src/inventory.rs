pub mod products;
// 👆 Inline
// 👆 inventory/products.rs
// 👆 inventory/products/mod.ds

pub use products::{Item, ProductCategory};

pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Ivan Inventory";

pub fn talk_to_manager() {
    println!(
        "Hey, {}, how's your coffee? What do you think of {:?}",
        MANAGER,
        ProductCategory::Ladder
    );
}
