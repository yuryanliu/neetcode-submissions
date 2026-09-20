impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        assert!(!nums.is_empty());
        let n = nums.len();
        let mut i = 0;
        let mut j = i + nums[i] as usize;
        while i != j && j < n - 1 {
            let mut l = j;
            for k in i+1..=j {
                l = l.max(k + nums[k] as usize);
            }
            i = j;
            j = l;
        }
        j >= n - 1
    }
}
