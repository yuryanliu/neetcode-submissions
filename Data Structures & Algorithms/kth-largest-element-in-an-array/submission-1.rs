impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut min_heap = BinaryHeap::<Reverse<i32>>::new();
        for num in nums {
            if (min_heap.len() as i32) < k {
                min_heap.push(Reverse(num));
            } else if let Some(top) = min_heap.peek() && top.0 < num {
                min_heap.pop();
                min_heap.push(Reverse(num));
            }
        }
        min_heap.pop().unwrap().0
    }
}
