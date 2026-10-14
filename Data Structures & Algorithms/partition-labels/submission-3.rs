impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        // convert it to intervals problem
        let mut intervals = vec![vec![]; 26];
        for (i, ch) in s.char_indices() {
            let d = (ch as u8 - b'a') as usize;
            if intervals[d].is_empty() {
                intervals[d].push(i);
                intervals[d].push(i);
            } else {
                intervals[d][1] = i;
            }
        }
        intervals.retain(|x| !x.is_empty());
        let mut res = vec![];

        if intervals.is_empty() {
            return res;
        }
        intervals.sort();
        let mut prev_interval = intervals[0].clone();
        for curr_interval in intervals.into_iter().skip(1) {
            if curr_interval[0] < prev_interval[1] {
                prev_interval[1] = prev_interval[1].max(curr_interval[1]);
            } else {
                res.push((prev_interval[1] - prev_interval[0] + 1) as i32);
                prev_interval = curr_interval;
            }
        }
        res.push((prev_interval[1] - prev_interval[0] + 1) as i32);
        res
    }
}
