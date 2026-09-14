impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        let mut pair = HashMap::new();
        pair.insert(b')', b'(');
        pair.insert(b'}', b'{');
        pair.insert(b']', b'[');

        for ch in s.as_bytes() {
            match ch {
                b'(' | b'{' | b'[' => {
                    stack.push(ch);
                }        
                b')' | b'}' | b']' => {
                    if let Some(left) = stack.pop() && 
                        let Some(right) = pair.get(&ch) &&
                        left == right {
                        continue;
                    } else {
                        return false;
                    }
                }        
                _ => {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}
