use std::io;

fn main() {
    // let mut full_name = String::from("Sylvester");
    // let last_name = String::from("Stallone");
    // full_name.push(' ');
    // full_name.push_str(&last_name);
    // println!("{last_name}");

    let first_name = String::from("Sylvester");
    let last_name = String::from("Stallone");

    // let full_name = first_name + &last_name;
    // println!("{full_name}");
    // println!("{first_name}");

    let icon = format!("{first_name} {last_name}");
    println!("{icon}");

    let mut music_genres = "                 Rock, Metal, Country, Rap      ";
    println!("{}", music_genres.trim());
    println!("{}", music_genres.trim_start());
    println!("{}", music_genres.trim_end());

    music_genres = music_genres.trim();
    println!("{}", music_genres.to_uppercase());
    println!("{}", music_genres.to_lowercase());

    println!("{}", music_genres.replace("a", "@"));

    let genres: Vec<&str> = music_genres.split(", ").collect();
    println!("{:?}", genres);

    let mut name = String::new();
    println!("what is your name?");
    match io::stdin().read_line(&mut name) {
        Ok(_) => println!("Hello, {}", name.trim()),
        Err(message) => println!("There was an error: {message}"),
    }
}
