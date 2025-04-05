use tasks::l6collection_task as l6collection;

fn main() {
    let numbers = vec![-1, -3, 5, -3, 7, 9];

    println!(
        "Positive numbers: {:?}",
        l6collection::filter_vector(&numbers, 0)
    );

    let word_count = l6collection::count_words("hello world hello");
    for (word, count) in &word_count {
        println!("{}: {}", word, count);
    }
}
