impl Solution {
    pub fn combination_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let (mut res, mut a) = (vec![], vec![]);
        Solution::backtracking(&mut res, &mut a, 0, 0, target, &nums);
        res
    }

    fn backtracking(res:&mut Vec<Vec<i32>>, a:&mut Vec<i32>, sum: i32, k: usize, target: i32, nums:&[i32]) {
        if sum == target {
            res.push(a.clone());
            return;
        }
        if sum > target || k == nums.len() {
            return;
        }

        for i in k..nums.len() {
            a.push(nums[i]);
            Solution::backtracking(res, a, sum+nums[i], i, target, nums);
            a.pop();
        }
    }
}
