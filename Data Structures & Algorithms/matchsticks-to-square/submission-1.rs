impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        let sum: i32 = matchsticks.iter().sum::<i32>();
        if sum % 4 != 0 || matchsticks.len() < 4 {
            return false;
        }
        matchsticks.sort_by(|a, b| b.cmp(a));
        let target = sum / 4;
        if matchsticks[0] > target {
            return false;
        }
        let mut sides = [0; 4];
        Solution::backtrack(0, &mut sides, target, &matchsticks)
    }

    fn backtrack(idx: usize, sides: &mut [i32; 4], target: i32, matchsticks: &[i32]) -> bool {
        if idx == matchsticks.len() {
            return true;
        }
        for i in 0..4 {
            if sides[i] + matchsticks[idx] <= target {
                sides[i] += matchsticks[idx];
                if Solution::backtrack(idx + 1, sides, target, matchsticks) {
                    return true;
                }
                sides[i] -= matchsticks[idx];
            }
            if sides[i] == 0 {
                break;
            }
        }
        false
    }
}