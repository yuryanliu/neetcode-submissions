impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
        let mut map = [0i32; 26];
        let mut global = 0;
        for ch in s1 {
            global += 1;
            map[(ch-b'a') as usize] += 1;
        }
        let (mut begin, mut end, mut count) = (0usize, 0usize, 0);
        while end < s2.len() {
            let i = (s2[end] - b'a') as usize;
            if map[i] == 0 {
                count -= 1;
            }
            map[i] -= 1;
            global -= 1;
            end += 1;

            while count < 0 && begin < end{
                let j = (s2[begin] - b'a') as usize;
                map[j] += 1;
                if map[j] == 0 {
                    count += 1;
                }
                global += 1;
                begin += 1;
            }

            if global == 0 {
                return true;
            }
        }
        false
    }
}
