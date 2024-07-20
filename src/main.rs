mod member;

use crate::member::Member;

fn main() {
    println!("Hello World!");
    let user = Member::new(
        String::from("Dundon"),
        String::from("Ben")
    );
    println!("{}", user)
}
