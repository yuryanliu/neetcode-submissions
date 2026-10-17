impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len(); assert!(m>0);
        let n = matrix[0].len(); assert!(n>0);
        let mut nums = vec![0; m];
        for i in 0..m {
            nums[i] = matrix[i][0];
        }
        let row = Solution::binary_search(&nums, 0i32, m as i32 - 1, target);
        if row < 0 {
            return false;
        }
        if nums[row as usize] == target {
            return true;
        }
        let col = Solution::binary_search(&matrix[row as usize][..], 0i32, n as i32 - 1, target);
        if col < 0 {
            return false;
        }
        if matrix[row as usize][col as usize] == target {
            return true;
        }
        false
    }

    fn binary_search(nums:&[i32], lo:i32, hi:i32, target: i32) -> i32 {
        let nums_at = |i: i32| nums[i as usize];
        let (mut red, mut blue) = (lo-1, hi+1);
        while red+1 != blue {
            let mi = red + (blue - red) / 2;
            let value = nums_at(mi);
            if value == target {
                return mi;
            } else if value > target {
                blue = mi;
            } else {
                red = mi;
            }
        }
        red
    }
}
