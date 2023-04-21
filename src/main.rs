use chrono::Utc;
use std::env::{self};

// entry object
// each entry will have a string and a date
// there will be a list of entries (an external file)
// each entry will render itself as this:
// X days without Y
// X represents the days calculated since the entry's tagged date
// Y represents the string associated.

struct Entry {
    date: i64,
    y: String,
}

impl Entry {
    fn days_elapsed(&self) -> i64 {
        (Utc::now().timestamp() - self.date) / 60 / 60 / 24
    }
}

fn list_entries(entries: &[Entry]) {
    for entry in entries.iter() {
        println!("{} days since {}", entry.days_elapsed(), entry.y);
    }
}

fn add_entry(entry_text: &str) -> Entry {
    Entry {
        y: entry_text.to_string(),
        date: Utc::now().timestamp(),
    }
}

fn main() {
    let mut entries: Vec<Entry> = Vec::new();
    let cli_args: Vec<String> = env::args().collect();
    if cli_args.len() > 1 {
        match cli_args[1].as_str() {
            "add" => {
                interactive_mode(&mut entries);
            }
            _ => list_entries(&entries),
        };
    }
}

fn interactive_mode(entries: &mut Vec<Entry>) {
    loop {
        println!("add new entry:");
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();
        let user_input_array: Vec<&str> = user_input.split(' ').collect();
        match user_input_array[0].trim_end() {
            "/list" => list_entries(entries),
            "/exit" => break,
            _ => {
                if user_input_array[0].len() > 1 {
                    entries.push(add_entry(&user_input));
                } else {
                    println!("please enter a valid entry")
                }
            }
        }
    }
}
