const OFFSET_X: [i32; 4] = [-1, 0, 1, 0];
const OFFSET_Y: [i32; 4] = [ 0, 1, 0,-1];
impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let n = grid.len(); assert!(n>0);
        let m = grid[0].len(); assert!(m>0);
        let mut queue = VecDeque::new();
        let mut fresh_fruits = 0;
        for j in 0..n {
            for i in 0..m {
                if grid[j][i] == 2 {
                    queue.push_back((i, j, 0));
                } else if grid[j][i] == 1 {
                    fresh_fruits += 1;
                }
            }
        }

        if fresh_fruits == 0 {
            return 0;
        }

        let mut last_minute = -1;
        while let Some((i, j, minute)) = queue.pop_front() {
            if minute > last_minute {
                last_minute = minute;
            }
            for k in 0..4 {
                let x = (OFFSET_X[k] + i as i32).max(0).min(m as i32 - 1) as usize;
                let y = (OFFSET_Y[k] + j as i32).max(0).min(n as i32 - 1) as usize;
                if grid[y][x] == 1 {
                    fresh_fruits -= 1;
                    grid[y][x] = 2;
                    queue.push_back((x, y, minute + 1));
                }
            }
        }
        if fresh_fruits == 0 {
            last_minute
        } else {
            -1
        }
    }
}
