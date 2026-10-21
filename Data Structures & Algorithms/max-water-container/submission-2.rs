impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        assert!(!heights.is_empty());
        let (mut lo, mut hi) = (0, heights.len() - 1);
        let mut global = 0;
        while lo < hi {
            let local = (hi - lo) as i32 * heights[lo].min(heights[hi]);
            global = global.max(local);
            if heights[lo] < heights[hi] {
                lo += 1;
            } else {
                hi -= 1;
            }
        }
        global
    }
}
