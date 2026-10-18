impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let n = numbers.len(); assert!(n>=2);
        let (mut lo, mut hi) = (0, n - 1);
        while lo < hi {
            let sum = numbers[lo] + numbers[hi];
            if sum == target {
                return vec![lo as i32 + 1, hi as i32 + 1];
            } else if sum < target {
                lo += 1;
            } else {
                hi -= 1;
            }
        }
        vec![-1, -1]
    }
}
