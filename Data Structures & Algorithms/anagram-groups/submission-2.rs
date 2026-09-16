impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut res:HashMap<[u8; 26], Vec<String>> = HashMap::new();
        for s in strs {
            let r = Solution::anagram(s.as_bytes());
            res.entry(r).or_default().push(s);
        }
        res.into_values().collect()
    }

    fn anagram(s: &[u8]) -> [u8; 26] {
        let mut r = [0u8; 26];
        for &c in s {
            r[(c - b'a') as usize] += 1;
        }
        r
    }
}
