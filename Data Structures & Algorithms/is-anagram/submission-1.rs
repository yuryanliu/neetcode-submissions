impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut letters = HashMap::new();
        for ch in s.chars() {
            letters.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
        }
        for ch in t.chars() {
            if let Some(counter) = letters.get_mut(&ch) {
                *counter -= 1;
                if *counter == 0 {
                    letters.remove(&ch);
                }
            } else {
                return false;
            }
        }
        letters.is_empty()
    }
}
