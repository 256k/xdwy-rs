use std::env::{self, Args};

// entry object
// each entry will have a string and a date
// there will be a list of entries (an external file)
// each entry will render itself as this:
// X days without Y
// X represents the days calculated since the entry's tagged date
// Y represents the string associated.
use chrono::Utc;

struct Entry {
    date: i64,
    y: String,
}

fn days_since(start_date: i64, end_date: i64) -> i64 {
    return (end_date - start_date) / 60 / 60 / 24;
}

fn add_entry(entry_text: &str) -> Entry {
    Entry {
        y: entry_text.to_string(),
        date: Utc::now().timestamp(),
    }
}

fn main() {
    // interactive_mode();
    let mut entries: Vec<Entry> = Vec::new();
    let cli_args: Vec<String> = env::args().collect();
    for _arg in cli_args.iter() {
        // println!("{}", arg);
    }
    if cli_args.len() > 1 {
        match cli_args[1].as_str() {
            "add" => {
                println!("you want me to add what, Boss?");
                entries.push(add_entry(&cli_args[2]));
            }
            "run" => {
                interactive_mode(&mut entries);
            }
            _ => println!("no add no foul"),
        };
    }
    for entry in entries.iter() {
        println!("{}", entry.y);
        println!("{}", entry.date / 60 / 60 / 24);
    }
    // let now = Utc::now().timestamp();
    // const TESTTIME: i64 = 1680411536;
    // let days = days_since(TESTTIME, now);
    // println!("days: {}", days );
    // let my_entry = add_entry("hello");
    // println!("{}", my_entry.y);
}

fn interactive_mode(entries: &mut Vec<Entry>) {
    println!("welcome to XdsY");
    loop {
        println!(">");
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();
        println!(" nothing? {}", user_input);
        match user_input.as_str().trim_end() {
            // TODO: convert user_input from a single string into an array 
            // of strings separated by spaces to be able to pass the 
            // second argument as the name of the entry in the add_entry fn
            "add" => entries.push(add_entry(&user_input)),
            "list" => list_entries(entries),
            "exit" => break,
            _ => println!("enter valid command"), 
        }
    }
}

fn list_entries(entries: &Vec<Entry>) {
    for entry in entries.iter() {
        println!("{} days since {}", entry.date, entry.y);
    }
}