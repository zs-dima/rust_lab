pub mod palindrome {
    // Check if a string is a palindrome
    pub fn is_palindrome(text: &str) -> bool {
        // Convert to vector of chars for efficient indexing
        let chars: Vec<char> = text.chars().collect();
        let length = chars.len();

        // Only need to check half the string
        for i in 0..length / 2 {
            if chars[i] != chars[length - 1 - i] {
                return false;
            }
        }
        true
    }

    pub fn palindrome_slice(text: &str) -> &str {
        // Convert to vector of chars for efficient indexing
        let chars: Vec<char> = text.chars().collect();
        let length = chars.len();

        // Only need to check half the string
        for i in 0..length / 2 {
            if chars[i] != chars[length - 1 - i] {
                return "is not a palindrome";
            }
        }
        &text[0..length / 2]
    }
}
