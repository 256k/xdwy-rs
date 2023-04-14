// entry object 
// each entry will have a string and a date
// there will be a list of entries (an external file)
// each entry will render itself as this:
// X days without Y
// X represents the days calculated since the entry's tagged date
// Y represents the string associated.
use chrono::Utc;

// struct Entry {
//     y: String,
//     date: i64,
// }

fn days_since(start_date: i64, end_date: i64) -> i64 {
    return (end_date - start_date) / 60/60/24
}

fn main() {
    let now = Utc::now().timestamp();
    const TESTTIME: i64 = 1680411536;
    let days = days_since(TESTTIME, now);
    println!("days: {}", days );
}
