mod member;
mod event;

use crate::member::Member;
use crate::event::Event;
use std::io;
use std::io::*;
use std::collections::LinkedList;
use indexmap::map::IndexMap;
use std::process;

const VERSION: &str = "0.1.0";
const DIVIDER: &str = "==================================================";

fn main() {
    let mut members_list: LinkedList<Member> = LinkedList::new();
    let mut events_list: LinkedList<Event> = LinkedList::new();
    let mut main_menu_options: IndexMap<&str, &fn()> = IndexMap::new();
    main_menu_options.insert("Exit", &(gracefulExit as fn()));
    init(&members_list, &events_list);
    doMenu(main_menu_options);
}

/**
* Loads members and events into memory from disk.
*/
fn init(members_list: &LinkedList<Member>, events_list: &LinkedList<Event>) {
    println!("{}", DIVIDER);
    println!("Welcome to Event Tracker version {}", VERSION);
    println!("Loading data from disk...");
    // TODO: Load pre-existing data, if any.

    // TODO: Initialise lists and relations

    println!("{}\n", DIVIDER);
}


/**
* Presents user with main menu
* TODO: Menu generic for nesting.
*/
fn doMenu(menu_options: IndexMap<&str, &fn()>) {
        println!("Please select a command: ");
        let mut index = 1;
        for (key, value) in &menu_options {
            println!("({}) {}", index, key);
            index += 1;
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().to_string();
        let inputInt: i32 = input.parse::<i32>().unwrap() - 1;
        let function: &str = menu_options.keys()[inputInt.try_into().unwrap()];
        menu_options.get(function).unwrap()();
}

/**
* Saves and exits the program
*/
fn gracefulExit() {
    // TODO: Save state back to disk

    println!("Exiting, goodbye!");
    println!("{}", DIVIDER);
}
