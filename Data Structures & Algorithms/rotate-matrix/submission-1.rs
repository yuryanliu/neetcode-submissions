impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n/2 {
            matrix.swap(i, n-1-i);
        } 
        for j in 0..n {
            for i in j..n {
                let t = matrix[j][i];
                matrix[j][i] = matrix[i][j];
                matrix[i][j] = t;
            }
        }       
    }
}
