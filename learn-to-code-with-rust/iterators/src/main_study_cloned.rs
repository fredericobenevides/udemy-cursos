fn main() {
    let teas = [
        String::from("Hot Earl Grey"),
        String::from("Iced Green"),
        String::from("Hot Matcha"),
    ];

    let more_teas: Vec<String> = teas.iter().cloned().collect();
    println!("{:?}", more_teas);

    let more_teas: Vec<String> = teas.iter().map(|tea| tea.clone()).collect();
    println!("{:?}", more_teas);

    let more_teas: Vec<String> = teas.clone().into_iter().collect();
    println!("{:?}", more_teas);

    let more_teas: Vec<String> = teas
        .iter()
        .filter(|tea| tea.contains("Hot"))
        .cloned()
        .collect();
    println!("{:?}", more_teas);
}
