const OFFSET_X: [i32; 4] = [-1, 0, 1, 0];
const OFFSET_Y: [i32; 4] = [ 0, 1, 0,-1];
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len(); assert!(m>0);
        let n = grid[0].len(); assert!(n>0);
        let mut visited = vec![];
        for _ in 0..m {
            visited.push(vec![false; n]);
        }
        let mut cc = 0;
        for j in 0..m {
            for i in 0..n {
                if !visited[j][i] && grid[j][i] == '1' {
                    Solution::dfs(&grid, &mut visited, i, j, n, m);
                    cc += 1;
                }
            }
        }

        cc
    }

    fn dfs(grid:&Vec<Vec<char>>, visited:&mut Vec<Vec<bool>>, i: usize, j: usize, n: usize, m: usize) {
        visited[j][i] = true;
        for k in 0..4 {
            let x = OFFSET_X[k] + i as i32;
            let y = OFFSET_Y[k] + j as i32;
            if x >= 0 && x < n as i32 && y >= 0 && y < m as i32 &&
                !visited[y as usize][x as usize] && grid[y as usize][x as usize] == '1' {
                    Solution::dfs(grid, visited, x as usize, y as usize, n, m);
                }
        }
    }
}
