const OFFSET_X: [i32; 4] = [-1, 0, 1, 0];
const OFFSET_Y: [i32; 4] = [ 0, 1, 0,-1];

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let m = board.len(); assert!(m > 0);
        let n = board[0].len(); assert!(n > 0);
        let word = word.as_bytes(); assert!(word.len() > 0);

        let mut visited = vec![];
        for j in 0..m {
            visited.push(vec![false; n]);
        }

        for j in 0..m {
            for i in 0..n {
                if word[0] == board[j][i] as u8 {
                    if Solution::backtracking(&mut visited, &board, word, 0, i, j, n, m) {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn backtracking(visited: &mut Vec<Vec<bool>>, board:&Vec<Vec<char>>, word: &[u8], mut k: usize, 
                    i: usize, j: usize, n: usize, m: usize) -> bool {
        if k == word.len() - 1 {
            return true;
        }
        k += 1;
        
        visited[j][i] = true;
        for l in 0..4 {
            let x = OFFSET_X[l] + i as i32;
            let y = OFFSET_Y[l] + j as i32;
            if x >= 0 && y >= 0 && x < n as i32 && y < m as i32 && !visited[y as usize][x as usize] &&
                word[k] == board[y as usize][x as usize] as u8 &&
                Solution::backtracking(visited, board, word, k, x as usize, y as usize, n, m) {
                return true;
            }
        }
        visited[j][i] = false;

        false
    }
}
