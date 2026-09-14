impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        let mut pair:HashMap<char, char> = [(')', '('), (']', '['), ('}', '{')].into();

        for ch in s.chars() {
            if let Some(right) = pair.get(&ch) {
                if let Some(left) = stack.pop() && left == *right {
                    continue;
                } else {
                    return false;
                }
            } else {
               stack.push(ch); 
            }
        }
        stack.is_empty()
    }
}
