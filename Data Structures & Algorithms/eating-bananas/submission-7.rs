impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        assert!(!piles.is_empty());
        let mut hi = *piles.iter().max().unwrap();
        let mut lo = 1i32;
        while lo <= hi {
            let mi = lo + (hi - lo) / 2;
            let val: i64= piles.iter().fold(0i64, |acc, pile| acc + ((pile + mi - 1) / mi) as i64);
            if val <= h as i64 {
                hi = mi - 1;
            } else {
                lo = mi + 1;
            }
        }
        lo
    }
}
