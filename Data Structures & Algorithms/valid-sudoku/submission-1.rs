impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let m = board.len(); assert!(m>0);
        let n = board[0].len(); assert!(n>0);
        // check rows
        for j in 0..m {
            let mut seen = vec![false; 9];
            for i in 0..n {
                let c = board[j][i];
                if c != '.' {
                    let d = (c as u8 - b'1') as usize;
                    if seen[d] {
                        return false;
                    } else {
                        seen[d] = true;
                    }
                }
            }
        }
        
        // check cols
        for i in 0..n {
            let mut seen = vec![false; 9];
            for j in 0..m {
                let c = board[j][i];
                if c != '.' {
                    let d = (c as u8 - b'1') as usize;
                    if seen[d] {
                        return false;
                    } else {
                        seen[d] = true;
                    }
                }
            }
        }

        // check 3x3
        for j in (0..m).step_by(3) {            
            for i in (0..n).step_by(3) {
                let mut seen = vec![false; 9];
                for y in 0..3 {
                    for x in 0..3 {
                        let c = board[j+y][i+x];
                        if c != '.' {
                            let d = (c as u8 - b'1') as usize;
                            if seen[d] {
                                return false;
                            } else {
                                seen[d] = true;
                            }
                        }
                    }
                }                
            }
        }

        true
    }
}
