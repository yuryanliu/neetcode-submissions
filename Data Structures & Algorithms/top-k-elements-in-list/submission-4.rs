impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut m = HashMap::new();
        for num in nums {
            *m.entry(num).or_default() += 1;
        }
        let v:Vec<Reverse<(i32, i32)>> = m.into_iter().map(|(key, val)| Reverse((val, key))).collect();
        let mut min_heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::from(v);
        while min_heap.len() as i32 > k  {
            min_heap.pop();
        }
        min_heap.into_iter().map(|Reverse((_, key))| key).collect()
    }
}
