impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        assert!(!nums.is_empty());
        let mut jumps = 0;
        let mut r = 0;
        let mut farest = 0;
        for l in 0..nums.len() - 1 {
            farest = farest.max(l + nums[l] as usize);
            if l == r {
                jumps += 1;
                r = farest;
            }
        }
        jumps
    }
}