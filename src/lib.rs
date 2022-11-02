mod front_of_house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    pub struct BreakFast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl BreakFast {
        pub fn summer(toast: &str) -> BreakFast {
            BreakFast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat() {
    let mut break_fast = crate::back_of_house::BreakFast::summer("Rye");
    break_fast.toast = String::from("Mutated");
}

struct User {
    userName: String,
    id: i32,
}

fn defineUser(mut user: User) -> i32 {
    println!("{}", user.userName);
    user.userName = String::from("Aoufi");
    user.id
}

fn main() {
    let mut user: User = User {
        userName: String::from("Aoufi Abderahmane"),
        id: 12,
    };
    defineUser(user); // ! ownership
    eat_at_restaurant();
}
