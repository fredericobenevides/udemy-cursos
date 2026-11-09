use std::io::stdin;

#[derive(Debug)]
struct Vault {
    password: String,
    treasure: String,
}

impl Vault {
    fn unlock<F>(self, procedure: F) -> Option<String>
    where
        F: FnOnce() -> String,
    {
        let user_password = procedure();
        if user_password == self.password {
            Some(self.treasure)
        } else {
            None
        }
    }
}

fn main() {
    let vault = Vault {
        password: String::from("topsecret"),
        treasure: String::from("Gold"),
    };

    // example 1 - FnOnce
    // let mut user_input = String::new();
    // println!("Please provide a password to crack the vault");
    // stdin().read_line(&mut user_input);
    // user_input = user_input.trim().to_string();

    // let hack = || user_input;

    // let extraction = vault.unlock(hack);
    // println!("{:?}", extraction);

    // example 2 - Fn()
    let hack = || {
        let mut user_input = String::new();
        println!("Please provide a password to crack the vault");
        stdin().read_line(&mut user_input);
        user_input.trim().to_string()
    };

    let extraction = vault.unlock(hack);
    println!("{:?}", extraction);
}
