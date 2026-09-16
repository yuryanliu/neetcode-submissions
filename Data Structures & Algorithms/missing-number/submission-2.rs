impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut x = nums.len() as i32;
        for i in 0..nums.len() {
            x ^= (i as i32) ^ nums[i];
        }
        x
    }
}
