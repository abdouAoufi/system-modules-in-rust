mod back_of_house;
mod front_of_house;
pub use crate::back_of_house::academy;
pub use crate::back_of_house::house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

fn deliver_order() {}

pub fn eat() {
    let mut break_fast = house::BreakFast::summer("HEllo");
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
    academy::register(String::from("Aoufi Abderahmane"), String::from("Middle"));
}
