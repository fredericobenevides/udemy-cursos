fn main() {
    let mut cereals = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];

    let first_two = &cereals[..2];
    println!("first two: {:?}", first_two);

    let mid_three = &cereals[1..4];
    println!("mid three: {:?}", mid_three);

    let last_three = &mut cereals[2..];
    println!("last three: {:?}", last_three);
    last_three[2] = String::from("Lucky Charms");
    println!("last three: {:?}", last_three);

    let cookie_crisp = &cereals[0];
    let cookie = &cookie_crisp[..6];
    println!("cookie: {}", cookie);

    let cocoa_puffs = &cereals[3];
    println!("cocoa_puffs: {}", cocoa_puffs);

    let puffs = &cocoa_puffs[6..];
    println!("puffs: {}", puffs);
}
