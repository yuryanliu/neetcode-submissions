impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let (mut left, mut right) = (vec![1i32; nums.len() + 1], vec![1i32; nums.len() + 1]);
        for i in 1..=nums.len() {
            left[i] = nums[i-1]*left[i-1];
        }
        for j in (0..nums.len()).rev() {
            right[j] = nums[j]*right[j+1];
        }
        let mut res = vec![0; nums.len()];
        for k in 0..nums.len() {
            res[k] = left[k]*right[k+1];
        }
        res
    }
}
