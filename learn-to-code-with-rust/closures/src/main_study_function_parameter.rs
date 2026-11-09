fn execute_thrice<F>(mut procedure: F)
where
    F: FnMut(),
{
    procedure();
    procedure();
    procedure();
}

fn bake_cake() {
    println!("Hello chocolate");
}

fn main() {
    let closure = || println!("I'm the boss");
    execute_thrice(closure);

    // let mut bosses = vec!["Boris"];
    // let closure = || {
    //     let employees = bosses;
    // };
    // execute_thrice(closure);

    let mut bosses = vec!["Boris"];
    let closure = || {
        bosses.push("Alexandra");
    };
    execute_thrice(closure);

    execute_thrice(bake_cake);

    let option: Option<Vec<String>> = None;
    let collection = option.unwrap_or_else(Vec::new);
    println!("{:?}", collection)
}
