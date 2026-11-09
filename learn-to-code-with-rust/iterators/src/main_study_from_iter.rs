use std::collections::HashSet;

#[derive(Debug)]
struct Playlist {
    songs: Vec<String>,
    users: HashSet<String>,
}

impl FromIterator<(String, String)> for Playlist {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut songs = Vec::new();
        let mut users = HashSet::new();

        for (song, user) in iter {
            songs.push(song);
            users.insert(user);
        }
        Self { songs, users }
    }
}

fn main() {
    // let fifty_numbers = 1..=50;

    // let results = Vec::from_iter(fifty_numbers.clone());
    // println!("{:?}", results);

    // let results = fifty_numbers.clone().collect::<Vec<i32>>();
    // println!("{:?}", results);

    // let unique_set: HashSet<_> = HashSet::from_iter(fifty_numbers.clone());
    // println!("{:?}", unique_set);

    // let results = fifty_numbers.clone().collect::<HashSet<i32>>();
    // println!("{:?}", results);

    // let chars = ['H', 'e', 'l', 'l', 'o'];
    // let greeting = String::from_iter(chars);
    // println!("{}", greeting);

    let songs = [
        (String::from("I Rust Go On"), String::from("Bob")),
        (String::from("A Rust of Wind"), String::from("Bob")),
        (String::from("A Rustworthy Man"), String::from("Sheila")),
    ];

    let playlist = Playlist::from_iter(songs.clone());
    println!("{:?}", playlist);

    let playlist = songs.clone().into_iter().collect::<Playlist>();
    println!("{:?}", playlist);

    // let playlist = songs.clone().iter().collect::<Playlist>();
}
