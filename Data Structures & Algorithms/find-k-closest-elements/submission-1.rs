impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut max_heap = BinaryHeap::new();
        for a in arr {
            let v = (a - x).abs();
            if (max_heap.len() as i32) < k {
                max_heap.push((v, a));
            } else if let Some(top) = max_heap.peek() && (top.0 > v || (top.0 == v && top.1 > a)) {
                max_heap.pop();
                max_heap.push((v, a));
            }
        }
        let mut res:Vec<i32> = max_heap.into_iter().map(|(_,a)| a).collect();
        res.sort();
        res
    }
}
