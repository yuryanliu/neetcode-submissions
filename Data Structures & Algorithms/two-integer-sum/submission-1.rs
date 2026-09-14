impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h = HashMap::new();
        for (j, n) in nums.iter().enumerate() {
            if let Some(i) = h.get(n) {
                return vec![*i as i32, j as i32];
            } else {
                h.insert(target-n, j);
            }
        }
        vec![]
    }
}
