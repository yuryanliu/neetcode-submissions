impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        assert!(!intervals.is_empty());
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut prev_end = intervals[0][1];
        let mut count = 0;
        for i in 1..intervals.len() {
            if intervals[i][0] < prev_end {
                count += 1;                
                prev_end = prev_end.min(intervals[i][1]);
            } else {
                prev_end = intervals[i][1];
            }
            //println!("count={}, prev_end={}, interval={:?}", count, prev_end, intervals[i]);
        }
        count
    }
}
