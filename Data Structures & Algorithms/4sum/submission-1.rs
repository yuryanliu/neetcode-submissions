impl Solution {
    // O(N^3)
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = vec![];
        let nums_at = | i: i32 | nums[i as usize] as i64;
        for i in 0..nums.len() as i32 - 3 {
            if i > 0 && nums_at(i) == nums_at(i-1) {
                continue;
            }
            for mut r in Solution::three_sum(&nums[i as usize + 1..], target as i64 - nums_at(i)) {
                r.push(nums_at(i) as i32);
                res.push(r);
            }
        }
        res
    }

    // O(N^2)
    fn three_sum(nums:&[i32], target: i64) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let nums_at = | i: i32 | nums[i as usize] as i64;
        for i in 0..nums.len() as i32 - 2 {
            if i > 0 && nums_at(i) == nums_at(i-1) {
                continue;
            }
            for mut r in Solution::two_sum(&nums[i as usize + 1..], target - nums_at(i)) {
                r.push(nums_at(i) as i32);
                res.push(r);
            }
        }
        res
    }

    // O(N)
    fn two_sum(nums:&[i32], target: i64) -> Vec<Vec<i32>> {
        let mut res = HashSet::new();
        let mut m = HashMap::new();
        for i in 0..nums.len() {
            if let Some(&val) = m.get(&(nums[i] as i64)) {
                res.insert(vec![val, nums[i]]);
            } else {
                m.insert(target - nums[i] as i64, nums[i]);
            }
        }
        res.into_iter().collect()
    }
}
