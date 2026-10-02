impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let (mut res, mut a) = (vec![], vec![]);
        Solution::backtracking(&mut res, &nums, &mut a, 0);
        res
    }

    fn backtracking(res:&mut Vec<Vec<i32>>, nums:&[i32], a:&mut Vec<i32>, k:usize) {
        res.push(a.clone());
        for i in k..nums.len() {
            if i > k && nums[i] == nums[i-1] {
                continue;
            }       
            a.push(nums[i]);
            Solution::backtracking(res, nums, a, i+1);
            a.pop();
        }
    }
}
