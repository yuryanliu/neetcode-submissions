impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }    
        intervals.sort();

        let mut res = vec![];
        let mut prev = intervals[0].clone();
        for interval in intervals.into_iter().skip(1) {
            if interval[0] <= prev[1] {
                prev[1] = prev[1].max(interval[1]);
            } else {
                res.push(prev);
                prev = interval;
            }
        }
        res.push(prev);
        res
    }
}
