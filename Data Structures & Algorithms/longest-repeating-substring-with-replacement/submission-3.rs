impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let (mut begin, mut end) = (0, 0);
        let (mut global, mut count) = (0, 0);
        let mut m = vec![0; 26];
        while end < s.len() {
            let e = (s[end] - b'A') as usize;
            m[e] += 1;
            end += 1;
            count = count.max(m[e]);

            while (end - begin) as i32 - count > k {
                let b = (s[begin] - b'A') as usize;
                m[b] -= 1;
                begin += 1;
            }

            global = global.max(end - begin);
        }
        global as i32
    }
}
