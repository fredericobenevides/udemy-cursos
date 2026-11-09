fn main() {
    let earnings = [4, 7, 9, 13];

    let sum = earnings
        .into_iter()
        .reduce(|total, current| total + current);
    println!("{:?}", sum);

    let earnings: [i32; 0] = [];

    let sum = earnings
        .into_iter()
        .reduce(|total, current| total + current);
    println!("{:?}", sum);

    let address_portion = [
        String::from("123 Elm Street"),
        String::from("Suburbia"),
        String::from("New Jersey"),
    ];

    println!("{}", address_portion.join(", "));

    let address = address_portion.into_iter().reduce(|accumulator, portion| {
        // accumulator.push_str(", ");
        // accumulator.push_str(&portion);
        // accumulator
        format!("{}, {}", accumulator, portion)
    });
    println!("{:?}", address);
}
