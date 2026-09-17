impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);
        while lo <= hi && nums[lo as usize] > nums[hi as usize] {
            let mi = lo + (hi - lo) / 2;
            if nums[lo as usize] <= nums[mi as usize] {
                lo = mi + 1;
            } else {
                hi = mi;
            } 
        }
        nums[lo as usize]
    }
}
