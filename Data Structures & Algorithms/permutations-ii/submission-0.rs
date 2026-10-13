impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let (mut res, mut a) = (HashSet::new(), vec![]);
        Solution::backtracking(&mut res, &mut a, 0, &nums);
        res.into_iter().collect()
    }

    fn backtracking(res:&mut HashSet<Vec<i32>>, a: &mut Vec<usize>, mut k: usize, nums:&[i32]) {
        if k == nums.len() {
            let mut r = vec![];
            for &i in &a[..] {
                r.push(nums[i]);
            }
            res.insert(r);
        } else {
            k += 1;
            for cand in Solution::construct_candidates(a, nums.len()) {
                a.push(cand);
                Solution::backtracking(res, a, k, nums);
                a.pop();
            }
        }
    }

    fn construct_candidates(a:&[usize], n: usize) -> Vec<usize> {
        let mut premutes = vec![false; n];
        for &i in a {
            premutes[i] = true;
        }
        let mut cands = vec![];
        for i in 0..n {
            if !premutes[i] {
                cands.push(i);
            }
        }
        cands
    }
}
