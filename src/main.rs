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

    let mut index = 1;

    for day in DAYS {
        println!("For the {day} day, my rust compiler gave to me:");
        for n in (0..index).rev() {
            println!("{}", ERRORS[n]);
        }
        index += 1;
    }
}
