use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    k : i32,
    max_heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut max_heap = BinaryHeap::new();
        for num in nums {
            KthLargest::op(&mut max_heap, num, k);
        }
        Self {
            k,
            max_heap,
        }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        KthLargest::op(&mut self.max_heap, val, self.k);
        self.max_heap.peek().unwrap().0
    }

    fn op(max_heap: &mut BinaryHeap<Reverse<i32>>, num: i32, k: i32) {
        if max_heap.len() < k as usize {
            max_heap.push(Reverse(num));
        } else if let Some(top) = max_heap.peek() && top.0 < num  {
            max_heap.pop();
            max_heap.push(Reverse(num));
        }
    }
}
