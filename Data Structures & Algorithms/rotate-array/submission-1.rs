impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = (((k % nums.len() as i32) + nums.len() as i32) % nums.len() as i32) as usize;
        Solution::reverse(&mut nums[..]);
        Solution::reverse(&mut nums[..k as usize]);
        Solution::reverse(&mut nums[k as usize..]);
    }

    fn reverse(nums:& mut [i32]) {
        for i in 0..nums.len() / 2 {
            nums.swap(i, nums.len() - 1 - i);
        }
    }
}
