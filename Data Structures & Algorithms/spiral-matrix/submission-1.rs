impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len() as i32; assert!(m>0);
        let n = matrix[0].len() as i32; assert!(n>0);
        let mut res = vec![];

        let (mut k, mut x, mut y) = (m * n, 0i32, 0i32);
        while k > 0 {
            // top
            for i in x..n-x {
                res.push(matrix[y as usize][i as usize]);
                k-=1;
            }
            if k == 0 {
                break;
            }

            // right
            for j in y+1..m-y {
                res.push(matrix[j as usize][(n-1-x) as usize]);
                k-=1;
            }
            if k == 0 {
                break;
            }

            // bottom
            for i in (x..n-1-x).rev() {
                res.push(matrix[(m-1-y) as usize][i as usize]);
                k-=1;
            }
            if k == 0 {
                break;
            }

            // left
            for j in (1+y..m-1-y).rev() {
                res.push(matrix[j as usize][x as usize]);
                k-=1;
            }
            if k == 0 {
                break;
            }

            x += 1;
            y += 1;
        }

        res
    }
}
