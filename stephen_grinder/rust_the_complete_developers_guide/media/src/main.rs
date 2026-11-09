mod content;

use content::catalog::Catalog;
use content::media::Media;

fn main() {
    let audiobook = Media::AudioBook {
        title: String::from("An Audiobook"),
    };

    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };

    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    // println!("{}", audiobook.description());
    // println!("{}", good_movie.description());
    // println!("{}", bad_book.description());

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    // println!("{:#?}", catalog.items.get(100));

    // let item = catalog.get_by_index(40);
    // println!("{:#?}", item);

    // match catalog.get_by_index(99) {
    //     MightHaveAValue::ThereIsAValue(value) => {
    //         println!("Item: #{:#?}", value);
    //     }
    //     MightHaveAValue::NoValueAvailable => {
    //         println!("No value here!");
    //     }
    // }

    // if let MightHaveAValue::ThereIsAValue(value) = catalog.get_by_index(99) {
    //     println!("Item in pattern match: {:#?}", value);
    // } else {
    //     println!("No value!!!!!!");
    // }

    // if let Some(value) = catalog.get_by_index_new(999) {
    //     println!("Item in pattern match with Some: {:#?}", value);
    // } else {
    //     println!("None value!!!!!!");
    // }

    let item = catalog.get_by_index_new(99);
    let placeholder = Media::Placeholder;
    // println!("#{:#?}", item.unwrap());
    // println!("#{:#?}", item.expect("expected there to be an item here!"));
    println!("{:#?}", item.unwrap_or(&placeholder));
}
