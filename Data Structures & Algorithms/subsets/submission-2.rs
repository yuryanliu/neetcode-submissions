impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut a, mut res) = (vec![], vec![]);
        Solution::backtracking(&nums, &mut a, 0, &mut res);
        res
    }

    fn backtracking(nums:&[i32], a:&mut Vec<i32>, mut k: usize, res:&mut Vec<Vec<i32>>) {
        if k == nums.len() {
            res.push(a.clone());
        } else {
            k += 1;
            let cands = [false, true];
            for cand in cands {
                if cand {
                    a.push(nums[k-1]);
                }
                Solution::backtracking(nums, a, k, res);
                if cand {
                    a.pop();
                }
            }
        }
    }

    /* bitmask
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut res = vec![];
        for i in 0..1<<n {
            let mut cand = vec![];
            for j in 0..n {
                if (i>>j)&0x1 != 0 {
                    cand.push(nums[j]);
                }
            }
            res.push(cand);
        }
        res
    }*/
}
