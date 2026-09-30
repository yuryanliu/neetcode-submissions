impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut nums: Vec<i32> = nums.into_iter().map(|n| n - 1).collect();
        let mut i = 0;
        while i < nums.len() {
            if nums[i] == i as i32 {
                i += 1;
            } else {
                let j = nums[i] as usize;
                if nums[j] == j as i32 {
                    return j as i32 + 1;
                } else {
                    nums.swap(i, j);
                }
            }
        }
        0
    }
}
