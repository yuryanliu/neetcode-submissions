impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<(i32, f64)> = vec![];
        for (position, speed) in position.into_iter().zip(speed).into_iter() {
            cars.push((position, (target-position) as f64 / speed as f64));
        }
        cars.sort_by(|a, b| a.0.cmp(&b.0));
        let mut monotonic_stack = vec![];
        for (_, hours) in cars {
            while let Some(top) = monotonic_stack.last() && hours >= *top {
                monotonic_stack.pop();
            }
            monotonic_stack.push(hours);
        }
        monotonic_stack.len() as i32
    }
}
