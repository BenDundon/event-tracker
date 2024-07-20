mod member;
mod event;

use crate::member::Member;
use crate::event::Event;

fn main() {
    let user = Member::new(
        String::from("Dundon"),
        String::from("Ben")
    );
    println!("{}", user);
    let event = Event::new(
        String::from("Tree Planting"),
        String::from("2024-07-12 12:30"),
        3.5
    );
    println!("{}", event)
}
