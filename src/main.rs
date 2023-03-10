fn main() {
    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "nineth",
        "tenth", "eleventh", "twelve",
    ];

    for day in DAYS {
        println!("For the {day} day, my rust compiler gave to me:");
    }
}
