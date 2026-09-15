impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        let (mut n, mut count) = (n, 0);
        while n != 0 {
            n &= n - 1;
            count += 1;
        }
        count
    }
}
