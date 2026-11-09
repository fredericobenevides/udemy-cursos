use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // let directory = fs::read_dir("./").unwrap_or_else(|error| {
    //     eprintln!("Could not read directory: {error}");
    //     process::exit(1);
    // });

    // for entry_result in directory {
    //     match entry_result {
    //         Ok(entry) => println!("{:?}", entry.path()),
    //         Err(error) => {
    //             eprintln!("Could not read entry: {}", error);
    //         }
    //     }
    // }

    // for entry_result in fs::read_dir("./")? {
    //     match entry_result {
    //         Ok(entry) => println!("{:?}", entry.path()),
    //         Err(error) => {
    //             eprintln!("Could not read entry: {}", error);
    //         }
    //     }
    // }

    // for entry_result in fs::read_dir("./")? {
    //     if let Ok(entry) = entry_result {
    //         println!("{:?}", entry.path());
    //     }
    // }

    for entry_result in fs::read_dir("./")? {
        let entry = entry_result?;
        let entry_name = entry.path();
        let metadata = fs::metadata(&entry_name)?;
        if metadata.is_file() {
            println!("{:?}\n-----------", entry_name);
            let contents = fs::read_to_string(&entry_name)?;
            println!("{}", contents);
        }
    }

    Ok(())
}
