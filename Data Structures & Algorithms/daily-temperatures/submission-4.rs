impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let (mut monotonic_stack, mut res) = (vec![], vec![0; temperatures.len()]);
        for (i, temperature) in temperatures.into_iter().enumerate() {
            if monotonic_stack.is_empty() {
                monotonic_stack.push((temperature, i));
            } else {
                while let Some(top) = monotonic_stack.last() && top.0 < temperature {
                    let top = monotonic_stack.pop().unwrap();
                    res[top.1] = (i - top.1) as i32;
                }
                monotonic_stack.push((temperature, i));
            }
        }
        res
    }
}
