impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s = s.as_bytes();
        let (mut res, mut a) = (vec![], vec![]);
        Solution::backtracking(&mut res, &mut a, s, 0);
        res
    }

    fn backtracking(res:&mut Vec<Vec<String>>, a:&mut Vec<String>, s:&[u8], mut k:usize) {
        if k == s.len() {
            res.push(a.clone());
        } else {
            for i in k..s.len() {
                if Solution::is_palindrome(&s[k..=i]) {
                    a.push(String::from_utf8(s[k..=i].to_vec()).unwrap());
                    Solution::backtracking(res, a, s, i+1);
                    a.pop();
                }
            }
        }
    }

    fn is_palindrome(s:&[u8]) -> bool {
        if s.is_empty() {
            return false;
        }
        let (mut l, mut r) = (0i32, s.len() as i32 - 1);
        while l < r && s[l as usize] == s[r as usize] {
            l += 1;
            r -= 1;
        }
        l >= r
    }
}
