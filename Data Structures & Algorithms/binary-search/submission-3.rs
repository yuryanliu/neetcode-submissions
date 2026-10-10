impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let nums_at = |i: i32| nums[i as usize];
        let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);
        while lo <= hi {
            let mi = lo + (hi - lo) / 2;
            if nums_at(mi) == target {
                return mi;
            } else if nums_at(mi) < target {
                lo = mi + 1;
            } else {
                hi = mi - 1;
            }
        }
        -1
    }
}
