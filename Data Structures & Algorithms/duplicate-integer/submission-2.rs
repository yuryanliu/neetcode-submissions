impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        //Solution 1:
        let mut s = HashSet::new();
        for n in nums {
            if s.contains(&n) {
                return true
            }
            s.insert(n);
        }
        false
    }
}
