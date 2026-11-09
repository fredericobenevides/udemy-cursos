fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];

    // let groups: (Vec<i32>, Vec<i32>) = numbers.into_iter().partition(|number| number % 2 == 0);
    // println!("{:?}", groups);

    let (evens, odds): (Vec<i32>, Vec<i32>) =
        numbers.into_iter().partition(|number| number % 2 == 0);
    println!("{:?} {:?}", evens, odds);
}
