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

fn add_entry(what: &str) -> Entry {
    Entry {
        y: what.to_string(),
        date: Utc::now().timestamp(),
    }
}

fn main() {
    let mut Entries: Vec<Entry> = Vec::new();
    let cli_args: Vec<String> = env::args().collect();
    for _arg in cli_args.iter() {
        // println!("{}", arg);
    }
    match cli_args[1].as_str() {
        "add" => {
            println!("you want me to add what, Boss?");
            Entries.push(add_entry(&cli_args[2]));
        }
        _ => println!("no add no foul"),
    };
    for entry in Entries.iter() {
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
