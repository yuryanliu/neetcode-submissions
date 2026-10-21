impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut counts = vec![0; 26];
        for task in tasks {
            counts[(task as u8 - b'A') as usize] += 1;
        }
        let mut max_heap = BinaryHeap::new();
        for count in counts {
            if count > 0 {
                max_heap.push(count);
            }
        }

        let mut intervals = 0;
        while !max_heap.is_empty() {
            let mut temp = vec![];
            for _ in 0..=n {
                if let Some(mut count) = max_heap.pop() {
                    count -= 1;
                    if count > 0 {
                        temp.push(count);
                    }
                } else if temp.is_empty() {
                    break;
                }
                intervals += 1;
            }
            for count in temp {
                max_heap.push(count);
            }
        }
        intervals
    }
}
