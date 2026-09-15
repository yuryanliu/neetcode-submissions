impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = vec![];
        for i in 0..=n {
            res.push(Solution::number_bits(i)); 
        }
        res
    }
    fn number_bits(mut n: i32) -> i32 {
        let mut c = 0;
        while n != 0 {
            n &= n - 1;
            c += 1;
        }
        c
    }
}
