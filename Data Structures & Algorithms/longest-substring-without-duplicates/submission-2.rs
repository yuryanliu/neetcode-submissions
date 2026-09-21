impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = vec![0; 128];
        let (mut d, mut count, mut begin, mut end) = (0, 0, 0, 0);
        let s = s.as_bytes();
        while end < s.len() {                       
            map[s[end] as usize] += 1;
            if map[s[end] as usize] > 1 {
                count += 1;
            } 
            end+=1;           

            while count > 0 {
                if map[s[begin] as usize] > 1 {
                    count -= 1;
                }
                map[s[begin] as usize] -= 1;
                begin+=1;
            }
            d = d.max(end - begin);
        }

        d as i32
    }
}
