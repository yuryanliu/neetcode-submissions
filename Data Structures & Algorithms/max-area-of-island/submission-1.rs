const OFFSET_X:[i32; 4] = [-1, 0, 1, 0];
const OFFSET_Y:[i32; 4] = [ 0, 1, 0,-1];
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len(); assert!(m>0);
        let n = grid[0].len(); assert!(n>0);
        let mut visited = vec![];
        for _ in 0..m {
            visited.push(vec![false; n]);
        }
        let mut max_area = 0;
        for j in 0..m {
            for i in 0..n {                
                if !visited[j][i] && grid[j][i] == 1 {
                    let mut area = 0;
                    Solution::dfs(&grid, &mut visited, i, j, n, m, &mut area);
                    max_area = max_area.max(area);
                }
            }
        }
        max_area
    }

    fn dfs(grid:&Vec<Vec<i32>>, visited:&mut Vec<Vec<bool>>, i: usize, j: usize, n: usize, m: usize, area:&mut i32) {
        visited[j][i] = true;
        *area += 1;

        for k in 0..4 {
            let x = (OFFSET_X[k] + i as i32).max(0).min(n as i32 - 1) as usize;
            let y = (OFFSET_Y[k] + j as i32).max(0).min(m as i32 - 1) as usize;
            if !visited[y][x] && grid[y][x] == 1 {
                Solution::dfs(grid, visited, x, y, n, m, area);
            }
        }
    }
}
