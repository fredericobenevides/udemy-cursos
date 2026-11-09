fn main() {
    let performers = ["Rustful Five", "Rust in Peace", "Rustin Bieber"];

    let last = performers.into_iter().last().unwrap();
    println!("{}", last);

    let second = performers.into_iter().nth(1).unwrap();
    println!("{}", second);

    let second_to_last = performers.into_iter().nth_back(2).unwrap();
    println!("{}", second_to_last);

    let target_index = performers
        .into_iter()
        .position(|element| element == "Rustin Bieber");
    println!("{:?}", target_index);

    let target_index = performers
        .into_iter()
        .position(|element| element == "Python Bieber");
    println!("{:?}", target_index);
}
