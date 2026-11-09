fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

fn longest2<'a>(first: &'a str, second: &str) -> &'a str {
    println!("The second is {}", second);
    first
}

fn main() {
    let orlando = String::from("Orlando");
    {
        let san_francisco = String::from("San Francisco");
        let result = longest(&orlando, &san_francisco);
        println!("{}", result);
    }
    println!("{}", orlando);

    let orlando = String::from("Orlando");
    let result = {
        let san_francisco = String::from("San Francisco");
        longest2(&orlando, &san_francisco)
    };
    println!("{}", result);
}
