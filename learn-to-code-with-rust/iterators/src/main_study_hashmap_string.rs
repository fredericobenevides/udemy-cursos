use std::collections::HashMap;

fn main() {
    let mut todos = HashMap::new();
    todos.insert("Pick up groceries", false);
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);

    for (_, completion_status) in &mut todos {
        *completion_status = true;
        // println!("Task: {}. Complete: {}", todo, completion_status);
    }

    println!("{:?}", todos);

    let seafood = "Oyster🦪";

    // for byte in seafood.bytes() {
    //     print!("{}/", byte);
    // }

    // for character in seafood.chars() {
    //     print!("{}/", character);
    // }

    println!("{}", seafood.bytes().len());
    println!("{}", seafood.chars().count());
}
