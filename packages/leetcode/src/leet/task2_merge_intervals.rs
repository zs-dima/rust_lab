pub mod merge_intervals {
    pub fn merge(vec: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        let mut merged = Vec::new();
        let mut sorted_intervals = vec.clone();
        sorted_intervals.sort_by(|a, b| a.0.cmp(&b.0));
        let mut current_interval = sorted_intervals[0].clone();
        for interval in sorted_intervals.iter().skip(1) {
            if interval.0 <= current_interval.1 {
                current_interval.1 = current_interval.1.max(interval.1);
            } else {
                merged.push(current_interval.clone());
                current_interval = interval.clone();
            }
        }
        merged.push(current_interval);
        merged
    }
}
