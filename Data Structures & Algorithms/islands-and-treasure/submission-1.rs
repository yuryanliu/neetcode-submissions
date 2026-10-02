const OFFSET_X:[i32; 4] = [-1, 0, 1, 0];
const OFFSET_Y:[i32; 4] = [ 0, 1, 0,-1];

impl Solution {
    pub fn islands_and_treasure(grid: &mut Vec<Vec<i32>>) {
        let m = grid.len(); assert!(m>0);
        let n = grid[0].len(); assert!(n>0);       
        for j in 0..m {
            for i in 0..n {
                if grid[j][i] == 0 {
                    Solution::bfs(grid, i, j, n, m);
                }
            }
        }
    }

    fn bfs(grid: &mut Vec<Vec<i32>>, i: usize, j:usize, n:usize, m:usize) {
        let mut queue = VecDeque::new();  
        queue.push_back((i, j));
        while let Some((i, j)) = queue.pop_front() {
            for k in 0..4 {
                let x = (OFFSET_X[k] + i as i32).max(0).min(n as i32 - 1) as usize;
                let y = (OFFSET_Y[k] + j as i32).max(0).min(m as i32 - 1) as usize;
                if grid[y][x] > grid[j][i] + 1 {
                    grid[y][x] = grid[j][i] + 1;
                    queue.push_back((x, y));
                }
            }
        }
    }  
}
