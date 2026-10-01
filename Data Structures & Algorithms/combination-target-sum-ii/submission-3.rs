impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let (mut candidates, mut res) = (candidates, vec![]);
        candidates.sort();
        let mut a = vec![];
        Solution::backtracking(&mut res, &candidates, &mut a, 0, 0, target);
        res
    }

    fn backtracking(res:&mut Vec<Vec<i32>>, nums:&[i32], a:&mut Vec<i32>, k: usize, sum:i32, target: i32) {
        if sum == target {
            res.push(a.clone());
            return;
        }
        for i in k..nums.len() {
            if nums[i] > target - sum {
                break;
            }
            if i > k && nums[i] == nums[i - 1] {
                continue;
            }
            a.push(nums[i]);
            Solution::backtracking(res, nums, a, i + 1, sum + nums[i], target);
            a.pop();
        }
    }
}