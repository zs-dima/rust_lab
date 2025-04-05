pub mod reverce_int {
    pub fn reverce(int: i32) -> i32 {
        let mut result = 0;
        let mut num = int.abs();
        while num != 0 {
            result = result * 10 + num % 10;
            num /= 10;
        }
        if int < 0 { -result } else { result }
    }
}
