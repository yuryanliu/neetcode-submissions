impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut intervals, mut new_interval_opt) = (intervals, Some(new_interval));
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut res = vec![];
        for interval in intervals {
            if let Some(mut new_interval) = new_interval_opt.take() {
                if new_interval[1] < interval[0] {
                    res.push(new_interval);
                } else if (interval[0] <= new_interval[0] && new_interval[0] <= interval[1]) ||
                    (new_interval[0] <= interval[0] && interval[0] <= new_interval[1]) {
                    new_interval[0] = new_interval[0].min(interval[0]);
                    new_interval[1] = new_interval[1].max(interval[1]);
                    new_interval_opt = Some(new_interval);
                    continue;
                } else {
                    new_interval_opt = Some(new_interval);
                }
            } 
            res.push(interval);      
        }
        if let Some(mut new_interval) = new_interval_opt.take() {
            res.push(new_interval);
        }
        res
    }
}
