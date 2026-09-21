impl Solution {
    pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut a = vec![];
        let mut res = vec![];
        Solution::backtracking(&mut res, &mut a, 0, &nums, target, 0);
        res
    }

    fn backtracking(res: &mut Vec<Vec<i32>>, a: &mut Vec<i32>, k: usize, nums:&[i32], target: i32, mut sum: i32) {
        if sum > target {
            return;
        } else if sum == target {
            res.push(a.clone());
        } else {
            for l in k..nums.len() {
                a.push(nums[l]);
                sum += nums[l];
                Solution::backtracking(res, a, l, nums, target, sum);
                a.pop();
                sum -= nums[l];
            }
        }
    }

}
