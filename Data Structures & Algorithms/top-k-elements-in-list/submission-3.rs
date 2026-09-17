impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        assert!(k>0);
        let mut freqs = HashMap::new();
        for num in nums {
            freqs.entry(num).and_modify(|count| *count += 1).or_insert(1);
        }
        let mut heap = BinaryHeap::<Reverse<(i32, i32)>>::new();
        for (num, count) in freqs {
            if heap.len() < k as usize {
                heap.push(Reverse((count, num)));
            } else if let Some(top) = heap.peek() && top.0.0 < count {
                heap.pop();
                heap.push(Reverse((count, num)));
            }
        }
        heap.iter().map(|v| v.0.1).collect()
    }
}
