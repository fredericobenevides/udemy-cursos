use std::collections::HashSet;

fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];

    // let my_iterator = numbers.iter();
    // let squares = my_iterator.map(|number: &i32| number.pow(2));

    // // println!("{:?}", my_iterator);
    // println!("{:?}", squares);

    // for number in squares {
    //     println!("Square: {}", number);
    // }

    for number in numbers.into_iter().map(|number: i32| number.pow(2)) {
        println!("Square: {}", number);
    }

    let numbers = vec![4, 8, 15, 16, 23, 42];
    let squares = numbers
        .iter()
        .map(|number: &i32| number.pow(2))
        .collect::<HashSet<i32>>();
    println!("{:?}", squares);
    println!("{:?}", numbers);

    let names = [
        String::from("Jimmy"),
        String::from("Cleveland"),
        String::from("Boris"),
    ];

    let name_lengths: Vec<usize> = names
        .iter()
        .map(|name| name.to_lowercase())
        .map(|name| name.replace("i", "@@"))
        .map(|name| name.len())
        .collect();

    println!("{:?}", name_lengths);
}
