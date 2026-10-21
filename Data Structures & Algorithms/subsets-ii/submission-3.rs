impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let (mut res, mut a) = (HashSet::new(), vec![]);
        Solution::backtracking(&mut res, &mut a, 0, &nums);
        res.into_iter().collect()   
    }

    fn backtracking(res:&mut HashSet<Vec<i32>>, a:&mut Vec<i32>, mut k: usize, nums:&[i32]) {
        if k == nums.len() {
            res.insert(a.clone());
        } else {
            k += 1;
            for cand in [true, false] {
                if cand {
                    a.push(nums[k-1]);
                }
                Solution::backtracking(res, a, k, nums);
                if cand {
                    a.pop();
                }
            }
        }
    }
}
