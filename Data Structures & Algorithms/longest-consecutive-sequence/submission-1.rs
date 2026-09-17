impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = HashSet::from_iter(nums.clone().into_iter());
        let mut global = 0;
        for num in nums {
            if !set.contains(&(num-1)) {
                let mut local = 1;
                let mut next = num + 1;
                while set.contains(&next) {
                    next += 1;
                    local += 1;
                }
                global = std::cmp::max(global, local);
            }
        }
        global
    }
}
