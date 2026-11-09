fn study() {
    let seasons = ["Spring", "Summer", "Fall", "Winter"];

    println!("{:?}", seasons);
    println!("{:#?}", seasons);

    dbg!(seasons);

    let employee = ("Molly", 3, "Marketing");

    // let name = employee.0;
    // let age = employee.1;
    // let department = employee.2;

    let (name, age, department) = employee;

    println!("Name: {}, age: {}, department: {}", name, age, department);
    println!("{employee:#?}");
}
