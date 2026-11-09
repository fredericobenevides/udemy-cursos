fn main() {
    let applicants = vec!["Rob", "Bob", "Cob", "Alex", "Piers", "John", "Dan"];

    // let winners = applicants
    //     .into_iter()
    //     .enumerate()
    //     .filter(|(index, _applicant)| index % 3 == 0)
    //     .map(|(_index, applicant)| applicant)
    //     .collect::<Vec<&str>>();
    // println!("{:?}", winners);

    let winners = applicants
        .into_iter()
        .enumerate()
        .filter_map(|(index, applicant)| {
            if index % 3 == 0 {
                Some(applicant)
            } else {
                None
            }
        })
        .collect::<Vec<&str>>();
    println!("{:?}", winners);
}
