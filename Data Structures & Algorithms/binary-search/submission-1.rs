impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
         let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);
         while lo <= hi {
            let mi = lo + (hi - lo) / 2;
            let value = nums[mi as usize];
            if value == target {
                return mi;
            } else if value < target {
                lo = mi + 1;
            } else {
                hi = mi - 1;
            }
         }
         return -1;
    }
}
