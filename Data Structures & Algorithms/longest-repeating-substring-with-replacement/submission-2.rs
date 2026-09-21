impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut map = vec![0; 128];
        let (mut d, mut count, mut begin, mut end) = (0, 0, 0, 0);
        let s = s.as_bytes();
        while end < s.len() {                                 
            map[s[end] as usize] += 1;
            count = count.max(map[s[end] as usize]);
            end+=1;           

            while (end - begin) as i32 - count > k {
                map[s[begin] as usize] -= 1;
                begin+=1;
            }
            d = d.max(end - begin);
        }

        d as i32
    }
}
