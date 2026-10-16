impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut begin, mut end) = (0, 0);
        let (mut global, mut sum) = (i32::MAX, 0i32);
        while end < nums.len() {
            sum += nums[end];
            end += 1;

            while sum >= target {
                global = global.min((end - begin) as i32);
                sum -= nums[begin];
                begin += 1;
            }
        }
        if global == i32::MAX {
            0
        } else {
            global
        }
    }
}
