impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0i32, heights.len() as i32 - 1);
        let mut global = 0;
        while lo < hi {
            let local = (hi - lo) * std::cmp::min(heights[lo as usize], heights[hi as usize]);
            global = std::cmp::max(local, global);
            if heights[lo as usize] < heights[hi as usize] {
                lo += 1;
            } else {
                hi -= 1;
            }
        }
        global
    }
}
