pub mod l6collection {
    use std::collections;

    pub fn filter_vector(vec: &Vec<i32>, threshold: i32) -> Vec<i32> {
        let mut filtered = Vec::new();
        for &num in vec {
            if num > threshold {
                filtered.push(num);
            }
        }
        filtered
    }

    pub fn count_words(text: &str) -> collections::HashMap<String, usize> {
        let mut word_count = collections::HashMap::new();
        for word in text.split_whitespace() {
            let count = word_count.entry(word.to_string()).or_insert(0);
            *count += 1;
        }
        word_count
    }
}

// #[cfg(test)]
// mod tests {
//     use std::collections;

//     #[test]
//     fn filter_vector() -> Result<(), String> {
//         let numbers = vec![-1, -3, 5, -3, 7, 9];
//         let result = super::filter_vector(&numbers, 0);
//         assert_eq!(result, vec![5, 7, 9]);
//         Ok(())
//     }

//     #[test]
//     fn count_words_test() -> Result<(), String> {
//         let text = "hello world hello";
//         let result = super::count_words(text);
//         let expected = vec![("hello".to_string(), 2), ("world".to_string(), 1)]
//             .into_iter()
//             .collect::<collections::HashMap<_, _>>();
//         assert_eq!(result, expected);
//         Ok(())
//     }
// }
