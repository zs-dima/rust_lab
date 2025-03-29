pub mod task3 {
    use std::num::ParseIntError;

    pub fn multiply_strings(str1: &str, str2: &str) -> Result<i32, ParseIntError> {
        let num1: i32 = str1.parse()?;
        let num2: i32 = str2.parse()?;
        Ok(num1 * num2)
    }
}
