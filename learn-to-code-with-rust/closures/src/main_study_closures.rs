fn main() {
    // immutable reference
    let multiplier = 5;

    let multiply_by = |value| value * multiplier;
    println!("{}", multiply_by(3 as u8));

    let numbers = vec![4, 8, 15, 16, 23, 42];
    println!("{:?}", numbers);

    let print_numbers = || println!("{:?}", numbers);
    print_numbers();
    print_numbers();
    print_numbers();
    println!("{:?}", numbers);

    // mutable reference
    let mut numbers = vec![4, 8, 15, 16, 23, 42];
    let mut add_number = || numbers.push(100);
    add_number();
    add_number();
    println!("{:?}", numbers);

    // closures with ownership
    let number = 13;
    let capture_number = || number;
    let a = capture_number();
    let b = capture_number();
    println!("{} {} {}", a, b, number);

    let first_name = String::from("Alice");
    let capture_string = || first_name;
    capture_string();
    // capture_string();
    // println!("{}", first_name);

    let first_name = String::from("Alice");
    let capture_string = || {
        let person = first_name;
        println!("{}", person);
    };
    capture_string();
    // capture_string();

    let first_name = String::from("Alice");
    let capture_string = || {
        println!("{}", first_name);
    };
    capture_string();
    capture_string();
    capture_string();
    println!("{}", first_name);

    let first_name = String::from("Alice");
    let last_name = String::from("Wonder");
    let capture_string = move || {
        println!("{} {}", first_name, last_name);
    };
    capture_string();
    capture_string();
    capture_string();
    // println!("{} {}", first_name, last_name);
}
