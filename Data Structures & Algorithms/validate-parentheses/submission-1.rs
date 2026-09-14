impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for ch in s.as_bytes() {
            match ch {
                b'(' | b'{' | b'[' => {
                    stack.push(ch);
                }
                b')' => {
                    if let Some(top) = stack.pop() &&
                        *top == b'(' {
                        continue;
                    } else {
                        return false
                    }
                }
                b'}' => {
                    if let Some(top) = stack.pop() &&
                        *top == b'{' {
                        continue;
                    } else {
                        return false
                    }
                }
                b']' => {
                    if let Some(top) = stack.pop() &&
                        *top == b'[' {
                        continue;
                    } else {
                        return false
                    }
                }
                _ => {}
            }
        }
        stack.is_empty()
    }
}
