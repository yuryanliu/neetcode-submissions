impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut counts = [0i32; 26];
        for ch in tasks {
            counts[(ch as u8 - b'A') as usize] += 1;
        }
        let mut max_heap = BinaryHeap::new();
        for i in 0..26 {
            if counts[i] > 0 {
                max_heap.push(counts[i]);
            }
        }

        let mut cycles = 0;
        while !max_heap.is_empty() {
            let mut tmp = vec![];
            for _ in 0..n+1 {
                if let Some(mut count) = max_heap.pop() {
                    cycles += 1;
                    count -= 1;
                    if count > 0 {
                        tmp.push(count);
                    }
                } else if !tmp.is_empty() {
                    cycles += 1;
                }
            }
            for count in tmp {
                max_heap.push(count);
            }
        }
        cycles
    }
}
