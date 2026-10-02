impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let (mut res, mut a) = (HashSet::new(), vec![]);
        Solution::backtracking(&mut res, &nums, &mut a, 0);
        res.into_iter().collect()
    }

    fn backtracking(res:&mut HashSet<Vec<i32>>, nums:&[i32], a:&mut Vec<i32>, mut k:usize) {
        if k == nums.len() {
            res.insert(a.clone());
        } else {
            k += 1;
            for cand in [false, true] {    
                if cand {
                    a.push(nums[k-1]);
                }
                Solution::backtracking(res, nums, a, k);
                if cand {
                    a.pop();
                }
            }
        }
    }
}
