impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut max_heap = BinaryHeap::from(stones);
        while max_heap.len() >= 2 {
            let x = max_heap.pop().unwrap();
            let y = max_heap.pop().unwrap();
            let d = x - y;
            if d > 0 {
                max_heap.push(d);
            }
        }
        if let Some(d) = max_heap.pop() {
            d
        } else {
            0
        }
    }
}
