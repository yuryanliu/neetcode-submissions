impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map:[Vec<u8>; 10] = [
            vec![], 
            vec![],
            vec![b'a', b'b', b'c'],
            vec![b'd', b'e', b'f'],
            vec![b'g', b'h', b'i'],
            vec![b'j', b'k', b'l'],
            vec![b'm', b'n', b'o'],
            vec![b'p', b'q', b'r', b's'],
            vec![b't', b'u', b'v'],
            vec![b'w', b'x', b'y', b'z'],
        ];
        let (mut res, mut a) = (vec![], vec![]);
        Solution::backtracking(&mut res, &mut a, 0, digits.as_bytes(), &map);
        res
    }

    fn backtracking(res:&mut Vec<String>, a:&mut Vec<u8>, mut k:usize, digits:&[u8], map:&[Vec<u8>]) {
        if k == digits.len() {
            if !a.is_empty() {
                res.push(String::from_utf8(a.clone()).unwrap());
            }
        } else {
            let d = (digits[k] - b'0') as usize;
            k += 1;            
            for &ch in &map[d] {
                a.push(ch);
                Solution::backtracking(res, a, k, digits, map);
                a.pop();
            }
        }
    }
}
