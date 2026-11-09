use warehouse::{FLOOR_SPACE, INVENTORY_MANAGER, ORDERS_MANAGER};

/// Get a summary of our current managers
fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of space",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );
}
