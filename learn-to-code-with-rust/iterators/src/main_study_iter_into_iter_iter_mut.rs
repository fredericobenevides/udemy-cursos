use std::collections::HashMap;

// OWNERSHIP
// for value in collection
// for value in collection.into_iter()

// IMMUTABLE REFERENCES
// for value in &collection
// for value in collection.iter()

// MUTABLE REFERENCES
// for value in &mut collection
// for value in collection.iter_mut()

fn main() {
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    // let my_iterator: std::vec::IntoIter<i32> = my_vector.into_iter();

    // for number in my_iterator {
    //     println!("{}", number);
    // }

    for number in my_vector {
        println!("{}", number);
    }

    let my_vector = vec![4, 8, 15, 16, 23, 42];
    // let my_iterator = my_vector.iter();

    // for number in my_iterator {
    //     println!("{}", number);
    // }

    for number in &my_vector {
        println!("{}", number);
    }

    let mut flavors = [
        String::from("Chocolate"),
        String::from("Vanilla"),
        String::from("Strawberry"),
    ];
    // let my_iterator = flavors.iter_mut();

    // for flavor in my_iterator {
    //     flavor.push_str(" Ice Cream");
    // }

    for flavor in &mut flavors {
        flavor.push_str(" Ice Cream");
    }

    println!("{:?}", flavors);
}
