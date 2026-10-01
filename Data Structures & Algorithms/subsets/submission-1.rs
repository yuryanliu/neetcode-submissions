impl Solution {
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
    }
}
