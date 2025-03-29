use std::num::ParseIntError;

use rust_lab::leet::leet1::task1;
use rust_lab::leet::leet2::task2;
use rust_lab::leet::leet3::task3;

#[test]
fn reverce_int_test() -> Result<(), String> {
    let numbers: i32 = 123;
    let result = task1::reverce_int(numbers);
    assert_eq!(result, 321);

    let numbers: i32 = -123;
    let result = task1::reverce_int(numbers);
    assert_eq!(result, -321);

    let numbers: i32 = -120;
    let result = task1::reverce_int(numbers);
    assert_eq!(result, -21);

    Ok(())
}

#[test]
fn merge_intervals_test() -> Result<(), String> {
    let numbers: Vec<(i32, i32)> = vec![(1, 3), (2, 6), (8, 10), (15, 18)];
    let result = task2::merge_intervals(&numbers);
    let expected = vec![(1, 6), (8, 10), (15, 18)];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn multiply_strings_test() -> Result<(), ParseIntError> {
    assert_eq!(task3::multiply_strings("3", "4")?, 12);
    assert_eq!(task3::multiply_strings("0", "5")?, 0);
    assert_eq!(task3::multiply_strings("-2", "3")?, -6);
    // assert_eq!(task3::multiply_strings("abc", "5")?, 0);
    Ok(())
}
