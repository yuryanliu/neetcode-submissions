impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut exists = HashSet::new();
        exists.insert(n);
        let mut n = n;
        while n != 0 && n != 1 {
            n = Solution::compute(n);
            if exists.contains(&n) {
                return false;
            }
            exists.insert(n);
        }
        n == 1
    }

    fn compute(mut n: i32) -> i32 {
        let mut sum = 0;
        while n != 0 {
            let d = n%10;
            n /= 10;
            sum += d*d;
        }
        sum
    }
}
