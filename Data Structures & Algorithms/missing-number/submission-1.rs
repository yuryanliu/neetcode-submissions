impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut x = 0;
        for i in 0..nums.len() {
            x ^= (i as i32) ^ nums[i];
        }
        x ^= nums.len() as i32;
        x
    }
}
