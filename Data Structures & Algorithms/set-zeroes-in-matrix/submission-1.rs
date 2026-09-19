impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len(); assert!(m>0);
        let n = matrix[0].len(); assert!(n>0);
        let (top_left, mut top, mut left) = (matrix[0][0] == 0, false, false);
        for i in 1..n{
            if matrix[0][i] == 0 {
                top = true;
                break;
            }
        } 
        for j in 1..m {
            if matrix[j][0] == 0 {
                left = true;
                break;
            }
        }
        for j in 1..m {
            for i in 1..n {
                if matrix[j][i] == 0 {
                    matrix[j][0] = 0;
                    matrix[0][i] = 0;
                }
            }
        }
        for i in 1..n {
            if matrix[0][i] == 0 {
                for j in 1..m {
                    matrix[j][i] = 0;
                }
            }
        }
        for j in 1..m {
            if matrix[j][0] == 0 {
                for i in 1..n {
                    matrix[j][i] = 0;
                }
            }
        }
        if top_left || top {
            for i in 0..n {
                matrix[0][i] = 0;
            }
        }
        if top_left || left {
            for j in 0..m {
                matrix[j][0] = 0;
            }
        }
    }
}
