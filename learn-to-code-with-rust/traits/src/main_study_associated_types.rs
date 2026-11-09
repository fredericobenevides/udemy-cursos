use std::ops::Add;

#[derive(Debug)]
struct Lunch {
    cost: f64,
}

// impl Add for Lunch {
//     type Output = f64;

//     fn add(self, rhs: Self) -> Self::Output {
//         self.cost + rhs.cost
//     }
// }

impl Add for Lunch {
    type Output = Lunch;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cost: self.cost + rhs.cost,
        }
    }
}

fn add_two_number<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let one = Lunch { cost: 19.99 };
    let two = Lunch { cost: 29.99 };
    println!("{:?}", one + two);

    let one = Lunch { cost: 19.99 };
    let two = Lunch { cost: 29.99 };
    let two_sum = add_two_number(one, two);
    println!("{:?}", two_sum);

    let integer_sum = add_two_number(1, 2);
    let float_sum = add_two_number(1.5, 2.4);
    println!("{}, {}", integer_sum, float_sum);
}
