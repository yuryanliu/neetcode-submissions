impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut map = [0i32; 26];
        for task in tasks {
            map[(task as u8 - b'A') as usize] += 1;
        }
        let mut max_heap:BinaryHeap<i32> = BinaryHeap::from(
            map.into_iter()
                .filter(|v| *v > 0)
                .collect::<Vec<_>>()
            );

        let mut cycles = 0;
        let mut temp = vec![];
        while !max_heap.is_empty() {
            for _ in 0..n+1 {
                if let Some(mut top) = max_heap.pop() {
                    cycles += 1;
                    top -= 1;
                    if top > 0 {
                        temp.push(top);
                    }
                } else if !temp.is_empty() {
                    cycles += 1;
                }
            }
            for count in temp.drain(..) {
                max_heap.push(count);
            }
        }

        cycles
    }
}
