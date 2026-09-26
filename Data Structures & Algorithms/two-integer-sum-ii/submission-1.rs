impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        assert!(!numbers.is_empty());
        let (mut lo, mut hi) = (0i32, numbers.len() as i32 -  1);
        while lo < hi {
            let sum = numbers[lo as usize] + numbers[hi as usize];
            if sum == target {
                return vec![lo + 1, hi + 1];
            } else if sum < target {
                lo += 1;
            } else {
                hi -= 1;
            }
        }
        vec![]
    }
}
