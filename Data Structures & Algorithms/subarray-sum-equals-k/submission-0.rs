impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut presums = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            presums[i+1] = presums[i]+nums[i];
        }
        let mut m:HashMap<i32, i32> = HashMap::new();
        let mut res = 0;
        for presum in presums {
            let target = presum - k;
            if let Some(&count) = m.get(&target) {
                res += count;
            }
            *m.entry(presum).or_default() += 1;
        }
        res
    }
}
