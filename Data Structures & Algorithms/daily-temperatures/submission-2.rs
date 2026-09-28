impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut monotonic_stack:Vec<(i32, usize)> = vec![];
        let mut res = vec![0i32; temperatures.len()];
        for (i, temperature) in temperatures.into_iter().enumerate() {
            while let Some(top) = monotonic_stack.last() &&
                temperature > top.0  {
                let (top, j) = monotonic_stack.pop().unwrap();
                res[j] = (i - j) as i32;
            }
            monotonic_stack.push((temperature, i));
        }
        while let Some((_, j)) = monotonic_stack.pop() {
            res[j] = 0;
        }
        res
    }
}
