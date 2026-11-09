fn main() {
    let fifty_numbers = 1..=50;

    for number in fifty_numbers.clone() {
        print!("{}/", number);
    }
    println!("\n");

    for number in fifty_numbers.clone().take(15) {
        print!("{}/", number);
    }
    println!("\n");

    for number in fifty_numbers.clone().rev().take(15) {
        print!("{}/", number);
    }
    println!("\n");

    for number in fifty_numbers.clone().skip(5).take(15) {
        print!("{}/", number);
    }
    println!("\n");

    for number in fifty_numbers.clone().take(15).skip(5) {
        print!("{}/", number);
    }
    println!("\n");

    for number in fifty_numbers.clone().take(15).skip(5).step_by(2) {
        print!("{}/", number);
    }
    println!("\n");
}
