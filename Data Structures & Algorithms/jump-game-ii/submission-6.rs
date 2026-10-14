impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let (mut l, mut r) = (0, 0);
        while r < nums.len() - 1 {
            let mut farthest = 0;
            for i in l..=r {
                farthest = farthest.max(i + nums[i] as usize);
            }
            l = r + 1;
            r = farthest;
            res += 1;
        }
        res
    }
}
