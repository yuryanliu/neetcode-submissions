impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 0.0 {
            return 0.0;
        }
        if n == 0 {
            return 1.0;
        }
        if n == 1 {
            return x;
        }
        if n == i32::MIN {
            return Solution::my_pow(x, n+1) / x;
        }
        if n < 0 {
            return 1.0 / Solution::my_pow(x, -n);
        }
        let r = Solution::my_pow(x, n/2);
        if n & 1 == 1 {            
            r*r*x
        } else {
            r*r
        }
    }
}
