#[derive(Debug)]
enum Tier {
    God,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(&self) {
        match self {
            Subscription::Free => {
                println!("You have limited access to the site");
            }
            Subscription::Basic(price, months) => {
                println!("You have limited access to the site's premium features for {price} for {months} months");
            }
            Subscription::Premium { tier } => {
                println!(
                    "You have full access to the site's premium features. Your tier is {tier:?}"
                )
            }
        }
    }
}

fn main() {
    let free = Subscription::Free;
    free.summarize();

    let basic = Subscription::Basic(9.99, 1);
    basic.summarize();

    let premium = Subscription::Premium {
        tier: Tier::Platinum,
    };
    premium.summarize();
}
