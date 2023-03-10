fn main() {
    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "nineth",
        "tenth", "eleventh", "twelve",
    ];
    const ERRORS: [&str; 12] = [
        "Using Sentinel Values.",
        "Hungarian Notation.",
        "An Abundance of Rc<RefCell<T>>",
        "Using the Wrong Integer Type.",
        "Unsafe - I Know What I'm Doing.",
        "Not Using Namespaces.",
        "Overusing Slice Indexing.",
        "Overusing Iterators.",
        "Not Leveraging Pattern Matching.",
        "Initialize After Construction.",
        "Defensive Copies.",
        "PANIC!",
    ];

    let mut index: usize = 1;
    let mut song_text = String::new();

    for day in DAYS {
        song_text.push_str("For the {day} day, my rust compiler gave to me:\n");
        for n in (0..index).rev() {
            song_text.push_str("{ERRORS[n]}\n");
        }
        index += 1;
    }
    println!("{song_text}");
}
