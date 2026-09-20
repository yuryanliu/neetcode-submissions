impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut pre_sum = vec![0i32; nums.len() + 1];
        for i in 0..nums.len() {
            pre_sum[i+1] = pre_sum[i]+nums[i];
        }
        let mut global = i32::MIN;
        let mut i = 0;
        for j in 1..pre_sum.len() {
            global = global.max(pre_sum[j] - pre_sum[i]);
            if pre_sum[j] < pre_sum[i] {
                i = j
            }
        }
        global
    }
}
