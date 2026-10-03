impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut counts = [0i32; 26];
        for ch in s.as_bytes() {
            counts[(ch-b'a') as usize] += 1;
        }
        let mut max_heap = BinaryHeap::new();
        for i in 0..26 {
            if counts[i] > 0 {
                max_heap.push((counts[i], b'a'+i as u8));
            }
        }

        let mut res = vec![];
        while !max_heap.is_empty() {
            let mut tmp = vec![];
            for _ in 0..2 {
                if let Some((mut count, ch)) = max_heap.pop() {
                    res.push(ch);
                    count -= 1;
                    if count > 0 {
                        tmp.push((count, ch));
                    }
                } else if !tmp.is_empty() {
                    return String::new();
                }
            }
            for (count, ch) in tmp {
                max_heap.push((count, ch));
            }
        }

        String::from_utf8(res).unwrap()
    }
}
