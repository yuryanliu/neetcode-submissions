impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        assert!(nums.len() >= 3);
        nums.sort();
        let mut res = vec![];
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for temp in Solution::two_sum(&nums[i+1..], -nums[i]) {
                res.push(vec![nums[i], temp[0], temp[1]]);
            }
        }
        res
    }

    fn two_sum(nums:&[i32], target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);
        let nums_at = |i: i32| nums[i as usize];
        while lo < hi {
            let sum = nums_at(lo) + nums_at(hi);
            //println!("{}+{}={} vs {}", lo, hi, sum, target);
            if sum == target {
                res.push(vec![nums_at(lo), nums_at(hi)]);
                lo += 1;
                hi -= 1;
                while lo < hi && nums_at(lo - 1) == nums_at(lo) {
                    lo += 1;
                }
                while lo < hi && nums_at(hi + 1) == nums_at(hi) {
                    hi -= 1;
                }
            } else if sum < target {
                lo += 1;
                while lo < hi && nums_at(lo - 1) == nums_at(lo) {
                    lo += 1;
                }
            } else {
                hi -=1;
                while lo < hi && nums_at(hi + 1) == nums_at(hi) {
                    hi -= 1;
                }
            }
        }
        res
    }
}
