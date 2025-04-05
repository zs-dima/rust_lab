// #[path = "../bin/l6collection.rs"]
// mod collection;
// use crate::collections;
// pub use crate::front_of_house::l6collection;

use std::collections;
use tasks::l6collection_task as tasks;

#[test]
fn filter_vector_test() -> Result<(), String> {
    let numbers: Vec<i32> = vec![-1, -3, 5, -3, 7, 9];
    let result = tasks::filter_vector(&numbers, 0);
    assert_eq!(result, vec![5, 7, 9]);
    Ok(())
}

#[test]
fn count_words_test() -> Result<(), String> {
    let text = "hello world hello";
    let result = tasks::count_words(text);
    let expected = vec![("hello".to_string(), 2), ("world".to_string(), 1)]
        .into_iter()
        .collect::<collections::HashMap<_, _>>();
    assert_eq!(result, expected);
    Ok(())
}
