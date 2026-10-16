impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len() as i32; assert!(m>0);
        let n = matrix[0].len() as i32; assert!(n>0);
        let (mut offset, mut k) = (0, m*n);

        let matrix_at = |j: i32, i: i32| matrix[j as usize][i as usize];

        let mut res = vec![];
        while k > 0 {
            for i in offset..n-offset {
                res.push(matrix_at(offset, i));
                k -= 1;
            }
            if k == 0 {
                break;
            }

            for j in offset+1..m-offset {
                res.push(matrix_at(j, n-offset-1));
                k -= 1;
            }
            if k == 0 {
                break;
            }

            for i in (offset..n-offset-1).rev() {
                res.push(matrix_at(m-offset-1, i));
                k -= 1;
            }
            if k == 0 {
                break;
            }

            for j in (offset+1..m-offset-1).rev() {
                res.push(matrix_at(j, offset));
                k -= 1;
            }

            offset += 1;
        }
        res
    }
}
