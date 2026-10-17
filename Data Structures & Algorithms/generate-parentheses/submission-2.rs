impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let (mut res, mut a) = (vec![], String::new());
        Solution::backtracking(&mut res, &mut a, n, 0, 0);
        res
    }

    fn backtracking(res:&mut Vec<String>, a:&mut String, n: i32, left: i32, right: i32) {
        if left == n {
            let mut b = a.clone();
            for _ in right..n {
                b.push(')');
            }
            res.push(b);
        } else {
            for is_left in Solution::construct_candidates(n, left, right) {
                if is_left {
                    a.push('(');
                } else {
                    a.push(')');
                }

                Solution::backtracking(res, a, n,
                    if is_left { left + 1} else {left},
                    if is_left { right } else { right + 1},
                );

                a.pop();
            }
        }
    }

    fn construct_candidates(n: i32, left: i32, right: i32) -> Vec<bool> {
        let mut is_lefts = vec![];

        if left < n {
            is_lefts.push(true);
        }
        if right < n && right < left {
            is_lefts.push(false);
        }

        is_lefts
    }
}
