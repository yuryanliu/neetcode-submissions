impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut res, mut a) = (vec![], vec![]);
        Solution::backtracking(&mut res, &nums, &mut a, 0);
        res
    }

    fn backtracking(res:&mut Vec<Vec<i32>>, nums:&[i32], a:&mut Vec<i32>, mut k:usize) {
        if k == nums.len() {
            res.push(a.clone());
        } else {
            k += 1;
            let cands = Solution::contruct_candidates(nums, a);
            for cand in cands {
                a.push(cand);
                Solution::backtracking(res, nums, a, k);
                a.pop();
            }
        }
    }

    fn contruct_candidates(nums:&[i32], a:&[i32]) -> Vec<i32> {
        let mut cands = vec![];
        let mut prems = HashSet::new();
        for &prem in a {
            prems.insert(prem);
        }
        for num in nums {
            if !prems.contains(num) {
                cands.push(*num);
            }
        }
        cands
    }
}
