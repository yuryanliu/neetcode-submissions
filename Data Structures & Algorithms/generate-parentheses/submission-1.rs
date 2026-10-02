impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let (mut res, mut a) = (vec![], vec![]);
        Solution::backtracking(&mut res, &mut a, 0, 0, n);
        res
    }

    fn backtracking(res:&mut Vec<String>, a:&mut Vec<u8>, mut l:i32, mut r:i32, n:i32) {
        if l > n || r > n {
            return;
        } else if l == n && r == n {
            res.push(String::from_utf8(a.clone()).unwrap());
        } else {
            let cands = Solution::construct_candidates(l, r, n);                     
            for cand in cands {
                if cand == b'(' {
                    l += 1;
                } else {
                    r += 1;
                };

                a.push(cand);
                Solution::backtracking(res, a, l, r, n);
                a.pop();

                if cand == b'(' {
                    l -= 1;
                } else {
                    r -= 1;
                };
            }
        }
    }

    fn construct_candidates(l: i32, r: i32, n: i32) -> Vec<u8> {
        let mut cands = vec![];
        if l < n {
            cands.push(b'(');
        }
        if r < l {
            cands.push(b')');
        }
        cands
    }
}
