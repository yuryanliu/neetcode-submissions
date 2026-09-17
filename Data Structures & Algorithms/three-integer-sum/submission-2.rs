impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();        
        let mut res = HashSet::new();
        for i in 0i32..nums.len() as i32 - 2 {
            let (mut j, mut k) = (i + 1, nums.len() as i32 - 1);
            let target = -nums[i as usize];
            while j < k {
                let val = nums[j as usize] + nums[k as usize];
                if val == target {
                    res.insert(vec![nums[i as usize], nums[j as usize], nums[k as usize]]);
                    j+=1;
                    k-=1;
                } else if val < target {
                    j+=1;
                } else {
                    k-=1;
                }
            }
        }
        res.into_iter().collect()
    }
}
