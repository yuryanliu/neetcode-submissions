impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let nums_at = |i: i32| nums[i as usize];
        let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);
        while lo <= hi {
            let mi = lo + (hi - lo) / 2;
            let mi_value = nums_at(mi);
            if mi_value == target {
                return mi;
            } else if nums_at(lo) <= mi_value {
                if  nums_at(lo) <= target && target < mi_value {
                    hi = mi - 1;
                } else {
                    lo = mi + 1;
                }
            } else {
                if mi_value < target && target <= nums_at(hi) {
                    lo = mi + 1;
                } else {
                    hi = mi - 1;
                }
            }
        }
        -1
    }
}
