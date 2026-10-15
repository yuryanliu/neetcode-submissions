impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut map = vec![0i32; 128];
        let (mut begin, mut end) = (0, 0);
        let (mut global, mut count) = (0, 0);

        while end < s.len() {
            if map[s[end] as usize] != 0 {
                count += 1;
            }
            map[s[end] as usize] += 1;
            end += 1;

            while count > 0 {
                if map[s[begin] as usize] != 1 {
                    count -= 1;
                }
                map[s[begin] as usize] -= 1;
                begin += 1;
            }
            global = global.max(end - begin);
        }

        global as i32
    }
}
