fn main() {
    let v1 = 1_337;
    let v2 = v1 as i16;

    let v3 = 1.234;

    let with_milk = true;
    let with_sugar = false;

    let is_my_type_of_coffee = with_milk && with_sugar;
    println!("{is_my_type_of_coffee}");

    let is_acceptable_coffee = with_milk || with_sugar;
    println!("{is_acceptable_coffee}");

    let numbers: [i8; 4] = [1, 2, 10, 20];
    println!("{numbers:#?}");

    let tupples = (12, 1.45, true);
    println!("{tupples:#?}");
}
