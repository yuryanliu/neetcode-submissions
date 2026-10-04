impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        assert!(!nums.is_empty());
        let (mut global, mut local) = (nums[0], nums[0]);
        for &num in nums.iter().skip(1) {
            local = std::cmp::max(local+num, num);
            global = global.max(local);
        }
        global
    }
}
