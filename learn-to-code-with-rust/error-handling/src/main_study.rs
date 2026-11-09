use std::fmt;
use std::io::{self, stdin, Read};
use std::process;

use std::fs::{self, read_to_string, File};

fn main() {
    // None.unwrap();

    // let a = 1;
    // panic!("Something went wrong");

    // process::exit(0);
    // println!("This will not print");

    // println!("Some status update");
    // eprintln!("Some error message");

    // let file = File::open("story.txt");
    // println!("{:#?}", file);

    // read_file();

    let file_result = read_file();

    match file_result {
        Ok(contents) => println!("{contents}"),
        Err(error) => {
            eprintln!("There was an error: {error:?}")
        }
    }

    let mut animals = vec!["Giraffe", "Monkey", "Zebra"];
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
}

// fn read_file() {
//     println!("Please enter the name of the file you like to read:");
//     let mut input = String::new();

//     let user_requested_file = stdin().read_line(&mut input);

//     if let Err(error) = user_requested_file {
//         eprintln!("Something went wrong collecting user input. The error was {error}");
//         process::exit(1)
//     }

//     let mut file = match File::open(&input.trim()) {
//         Ok(file) => file,
//         Err(error) => {
//             eprintln!(
//                 "Something went wrong reading the file. The error was {:?}",
//                 error
//             );
//             process::exit(1)
//         }
//     };

//     let mut file_contents = String::new();
//     let read_operations = file.read_to_string(&mut file_contents);

//     if let Err(error) = read_operations {
//         eprintln!(
//             "Something went wrong reading the file as a string. The error was {:?}",
//             error
//         )
//     }

//     println!("{file:?}");
//     println!("{file_contents:?}");
// }

// fn read_file() -> Result<String, io::Error> {
//     println!("Please enter the name of the file you like to read:");
//     let mut input = String::new();

//     let user_requested_file = stdin().read_line(&mut input);

//     if let Err(error) = user_requested_file {
//         return Err(error);
//     }

//     let mut file = match File::open(&input.trim()) {
//         Ok(file) => file,
//         Err(error) => return Err(error),
//     };

//     let mut file_contents = String::new();
//     let read_operations = file.read_to_string(&mut file_contents);

//     if let Err(error) = read_operations {
//         return Err(error);
//     }

//     Ok(file_contents)
// }

// fn read_file() -> Result<String, io::Error> {
//     println!("Please enter the name of the file you like to read:");

//     let mut input = String::new();
//     stdin().read_line(&mut input)?;

//     let mut file_contents = String::new();
//     File::open(&input.trim())?.read_to_string(&mut file_contents)?;

//     Ok(file_contents)
// }

fn read_file() -> Result<String, io::Error> {
    println!("Please enter the name of the file you like to read:");

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    fs::read_to_string(input.trim())
}

fn length_of_last_element(input: &mut Vec<&str>) -> Option<usize> {
    let last_element = input.pop()?;
    Some(last_element.len())
}
