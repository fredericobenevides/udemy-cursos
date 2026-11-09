fn main() {
    let spreadsheet = vec![[100, 200, 300], [123, 456, 789], [987, 654, 321]];

    let values: Vec<i32> = spreadsheet.into_iter().flatten().collect();
    println!("{:?}", values);

    let attendees = [
        "Bob, Mary, Kevin",
        "Mike, Robbie, Matt, Austin",
        "Piers, Liam",
    ];

    let attendees: Vec<&str> = attendees
        .iter()
        .map(|group| group.split(", "))
        .flatten()
        .collect();
    println!("{:?}", attendees);

    let attendees: Vec<&str> = attendees
        .iter()
        .flat_map(|group| group.split(", "))
        .collect();
    println!("{:?}", attendees);
}
