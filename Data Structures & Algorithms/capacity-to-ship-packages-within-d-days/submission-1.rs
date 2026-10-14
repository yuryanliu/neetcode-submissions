impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut hi:i32 = weights.iter().sum::<i32>() +1;
        let mut lo:i32 = *weights.iter().min().unwrap()-1;
        while lo+1 != hi {
            let mi = lo + (hi - lo) / 2;
            let takes = Solution::days(&weights, mi);
            if takes <= days {
                hi = mi;
            } else {
                lo = mi;
            }
        }
        hi
    }

    fn days(weights:&[i32], capacity: i32) -> i32 {
        let (mut d, mut sum) = (0, 0);
        for &weight in weights {
            if weight > capacity {
                return i32::MAX;
            }
            if sum + weight > capacity {
                d += 1;
                sum = weight;
            } else {
                sum += weight;
            }
        }
        if sum > 0 {
            d + 1
        } else {
            d
        }
    }
}
