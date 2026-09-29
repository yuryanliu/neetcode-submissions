impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut v = vec![0i32; matrix.len()];
        for j in 0..v.len() {
            v[j] = matrix[j][0];
        }
        let row = Solution::binary_search(&v, target, 0i32, v.len() as i32 - 1);
        if row < 0 {
            return false;
        } else if v[row as usize] == target {
            return true;
        }

        let col = Solution::binary_search(&matrix[row as usize], target, 0i32, matrix[0].len() as i32 - 1);
        if col < 0 {
            return false;
        } else if matrix[row as usize][col as usize] == target {
            return true;
        }
        false
    }

    fn binary_search(v:&[i32], target: i32, mut lo: i32, mut hi: i32) -> i32 {
        while lo <= hi {
            let mi = lo + (hi - lo) / 2;
            let val = v[mi as usize];
            if val == target {
                return mi;
            } else if val < target {
                lo = mi + 1;
            } else {
                hi = mi - 1;
            }
        }
        hi
    }
}
