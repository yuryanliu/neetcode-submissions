impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut max_heap = BinaryHeap::new();
        for a in arr {
            let v = (a - x).abs();
            max_heap.push((v, a));
            if max_heap.len() as i32 > k {
                max_heap.pop();
            }
        }
        let mut res:Vec<i32> = max_heap.into_iter().map(|(_,a)| a).collect();
        res.sort();
        res
    }
}
