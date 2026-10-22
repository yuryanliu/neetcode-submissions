const OFFSET_X: [i32; 4] = [-1, 0, 1, 0];
const OFFSET_Y: [i32; 4] = [ 0, 1, 0,-1];
impl Solution {
    pub fn islands_and_treasure(grid: &mut Vec<Vec<i32>>) {
        let n = grid.len(); assert!(n>0);
        let m = grid[0].len(); assert!(m>0);
        let mut queue = VecDeque::new();
        for j in 0..n {
            for i in 0..m {
                if grid[j][i] == 0 {
                    queue.push_back((i, j))
                }
            }
        }

        while let Some((i, j)) = queue.pop_front() {
            for k in 0..4 {
                let x = (i as i32 + OFFSET_X[k]).max(0).min(m as i32 - 1) as usize;
                let y = (j as i32 + OFFSET_Y[k]).max(0).min(n as i32 - 1) as usize;
                if grid[y][x] != -1 && grid[y][x] > grid[j][i] + 1 {
                    grid[y][x] = grid[j][i] + 1;
                    queue.push_back((x, y));
                }
            }
        }
    }
}
