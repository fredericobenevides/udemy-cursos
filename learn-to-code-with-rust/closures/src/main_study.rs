fn main() {
    let multiply_by = || 5 as u8;
    println!("{}", multiply_by());

    let multiplier = 5;

    let multiply_by = |value: i32| -> i32 {
        return value * multiplier;
    };
    println!("{}", multiply_by(5));

    let multiply_by = |value: i32| value * multiplier;
    println!("{}", multiply_by(5));

    let multiplier = 5;
    let multiply_by = |value| value * multiplier;
    println!("{}", multiply_by(5 as u8));
    println!("{}", multiply_by(1000));

    let product = |a: i32, b: i32| -> i32 {
        println!("Calculating product for you");
        return a * b;
    };
    println!("{}", product(3, 10));
    println!("{}", product(5, 8));
}
