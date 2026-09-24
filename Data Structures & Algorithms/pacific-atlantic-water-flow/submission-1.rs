const OFFSET_X: [i32; 4] = [-1, 0, 1, 0];
const OFFSET_Y: [i32; 4] = [ 0, 1, 0,-1];
impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len(); assert!(m>0);
        let n = heights[0].len(); assert!(n>0);
        let mut visited = vec![];
        for _ in 0..m {
            visited.push(vec![0i32; n]);
        }
        //Top, Left
        for i in 0..n {
            if visited[0][i] & 0x1 == 0 {
                Solution::dfs(&heights, &mut visited, i, 0, n, m, 0x1);
            }
        }
        for j in 0..m {
            if visited[j][0] & 0x1 == 0 {
                Solution::dfs(&heights, &mut visited, 0, j, n, m, 0x1);
            }
        }

        //Bottom, Right
        for i in 0..n {
            if visited[m-1][i] & 0x2 == 0 {
                Solution::dfs(&heights, &mut visited, i, m-1, n, m, 0x2);
            }
        }
        for j in 0..m {
            if visited[j][n-1] & 0x2 == 0 {
                Solution::dfs(&heights, &mut visited, n-1, j, n, m, 0x2);
            }
        }

        let mut res = vec![];
        for j in 0..m {
            for i in 0..n {
                if visited[j][i] == 0x3 {
                    res.push(vec![j as i32, i as i32]);
                }
            }
        }
        res
    }

    fn dfs(heights:&Vec<Vec<i32>>, visited:&mut Vec<Vec<i32>>, i: usize, j: usize, n: usize, m: usize, d:i32) {
        visited[j][i] += d;
        for k in 0..4 {
            let x = OFFSET_X[k] + i as i32;
            let y = OFFSET_Y[k] + j as i32;
            if x >= 0 && x < n as i32 && y >= 0 && y < m as i32 &&
               visited[y as usize][x as usize] & d == 0 && heights[y as usize][x as usize] >= heights[j][i] {
                    Solution::dfs(heights, visited, x as usize, y as usize, n, m, d);
               }
        }
    }
}
