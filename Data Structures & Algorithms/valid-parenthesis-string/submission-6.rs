impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let (mut stack_left, mut stack_star) = (vec![], vec![]);
        for (i, ch) in s.char_indices() {
            if ch == '(' {
                stack_left.push(i);
            } else if ch == '*' {
                stack_star.push(i);
            } else { // ')'
                if !stack_left.is_empty() {
                    stack_left.pop();
                } else if !stack_star.is_empty() {
                    stack_star.pop();
                } else {
                    return false;
                }
            }
        }
        while !stack_left.is_empty() && !stack_star.is_empty() {
            if let (Some(left), Some(star)) = (stack_left.pop(), stack_star.pop()) &&
                left > star {
                return false;
            }
        }

        stack_left.is_empty()
    }
}
