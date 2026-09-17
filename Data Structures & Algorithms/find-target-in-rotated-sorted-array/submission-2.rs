impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);
        while lo <= hi {
            let mi = lo + (hi - lo) / 2;
            if nums[mi as usize] == target {
                return mi;
            } else if nums[lo as usize] <= nums[mi as usize] {
                if nums[lo as usize] <= target && target < nums[mi as usize] {
                    hi = mi - 1;
                } else {
                    lo = mi + 1;
                }
            } else {
                if nums[mi as usize] < target && target <= nums[hi as usize] {
                    lo = mi + 1;
                } else {
                    hi = mi - 1;
                }
            }
        }
        -1
    }
}
