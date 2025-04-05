use std::num::ParseIntError;

use leetcode::merge_intervals;
use leetcode::multiply_strings;
use leetcode::reverce_int;

#[test]
fn reverce_int_test() -> Result<(), String> {
    let numbers: i32 = 123;
    let result = reverce_int::reverce(numbers);
    assert_eq!(result, 321);

    let numbers: i32 = -123;
    let result = reverce_int::reverce(numbers);
    assert_eq!(result, -321);

    let numbers: i32 = -120;
    let result = reverce_int::reverce(numbers);
    assert_eq!(result, -21);

    Ok(())
}

#[test]
fn merge_intervals_test() -> Result<(), String> {
    let numbers: Vec<(i32, i32)> = vec![(1, 3), (2, 6), (8, 10), (15, 18)];
    let result = merge_intervals::merge(&numbers);
    let expected = vec![(1, 6), (8, 10), (15, 18)];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn multiply_strings_test() -> Result<(), ParseIntError> {
    assert_eq!(multiply_strings::multiply("3", "4")?, 12);
    assert_eq!(multiply_strings::multiply("0", "5")?, 0);
    assert_eq!(multiply_strings::multiply("-2", "3")?, -6);
    // assert_eq!(multiply_strings::multiply("abc", "5")?, 0);
    Ok(())
}
