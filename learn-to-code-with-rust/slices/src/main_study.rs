fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day");
}

fn study() {
    // example
    // let action_hero = "Arnold Schwarzenegger";
    // let first_name = &action_hero[0..6];
    // println!("{first_name}");

    // let last_name = &action_hero[7..21];
    // println!("{last_name}");

    // example
    // let first_name = {
    //     let action_hero = "Arnold Schwarzenegger";
    //     &action_hero[0..6]
    // };
    // println!("{first_name}");

    // let food = "🍕";
    // println!("{}", food.len());
    // let pizza_slice = &food[0..3];
    // println!("{}", pizza_slice.len());

    // example
    // let action_hero = String::from("Arnold Schwarzenegger");
    // let first_name = &action_hero[..6];
    // println!("Hist first name is {first_name}");

    // let last_name = &action_hero[7..];
    // println!("His last name is {last_name}");

    // let full_name = &action_hero[..];
    // println!("His full name is {full_name}");

    // example
    // let action_hero = String::from("Arnold Schwarzenegger");
    // do_hero_stuff(&action_hero);
    // let another_action_hero = "Sylvester Stallone";
    // do_hero_stuff(&another_action_hero);

    // example
    // let values = [4, 8, 15, 16, 23, 42];
    // let my_slice = &values[..4];
    // println!("{my_slice:?}");

    // let my_slice = &values[2..4];
    // println!("{my_slice:?}");

    // let my_slice = &values[2..];
    // println!("{my_slice:?}");

    // let my_slice = &values[..];
    // println!("{my_slice:?}");

    // let my_slice = &values;
    // println!("{my_slice:?}");

    // example
    // let values = [4, 8, 15, 16, 23, 42];

    // let regular_reference = &values;
    // print_length(regular_reference);

    // let slice_of_three = &values[..3];
    // print_length(slice_of_three);

    // example
    let mut my_array = [10, 15, 20, 25, 30];
    let my_slice = &mut my_array[2..4];
    println!("My slice: {:?}", my_slice);

    my_slice[0] = 100;
    println!("My slice: {:?}", my_slice);
    println!("My array: {:?}", my_array);
}

fn print_length(reference: &[i32]) {
    println!("{}", reference.len());
}
