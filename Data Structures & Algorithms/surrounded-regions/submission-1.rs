const OFFSET_X: [i32; 4] = [-1, 0, 1, 0];
const OFFSET_Y: [i32; 4] = [ 0, 1, 0,-1];
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let n = board.len(); assert!(n>0);
        let m = board[0].len(); assert!(m>0);
        let mut queue = VecDeque::new();
        for j in 0..n {
            if board[j][0] == 'O' {
                board[j][0] = 'Z';
                queue.push_back((0, j));
            }
            if board[j][m-1] == 'O' {
                board[j][m-1] = 'Z';
                queue.push_back((m-1, j));
            }
        }
        for i in 0..m {
            if board[0][i] == 'O' {
                board[0][i] = 'Z';
                queue.push_back((i, 0));
            }
            if board[n-1][i] == 'O' {
                board[n-1][i] = 'Z';
                queue.push_back((i, n-1));
            }
        }
        while let Some((i, j)) = queue.pop_front() {
            for k in 0..4 {
                let x = (OFFSET_X[k] + i as i32).max(0).min(m as i32 - 1) as usize;
                let y = (OFFSET_Y[k] + j as i32).max(0).min(n as i32 - 1) as usize;
                if board[y][x] == 'O' {
                    board[y][x] = 'Z';
                    queue.push_back((x, y));
                }
            }
        }
        for j in 0..n {
            for i in 0..m {
                if board[j][i] == 'O' {
                    board[j][i] = 'X';
                }
            }
        }
        for j in 0..n {
            for i in 0..m {
                if board[j][i] == 'Z' {
                    board[j][i] = 'O';
                }
            }
        }
    }
}
