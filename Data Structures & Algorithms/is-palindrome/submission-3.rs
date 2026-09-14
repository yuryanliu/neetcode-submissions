impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let (mut i, mut j) = (0 as i32, s.len() as i32 - 1);
        while i < j {
            while i < j && !s[i as usize].is_ascii_alphanumeric() {
                i+=1;
            }
            while i < j && !s[j as usize].is_ascii_alphanumeric() {
                j-=1;
            }
            if s[i as usize].to_ascii_lowercase() == s[j as usize].to_ascii_lowercase() {
                i+=1;
                j-=1;
            } else {
                return false;
            }
        }
        i>=j
    }
}
