impl Solution {
    pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            nums[i] -= 1;
        }
        let mut i = 0;
        while i < nums.len() {
            if nums[i] == i as i32 {
                i += 1;
            } else {
                let j = nums[i] as usize;
                if nums[i] == nums[j] {
                    return nums[i] + 1;
                }
                nums.swap(i, j);
            }
        }
        -1
    }
}
