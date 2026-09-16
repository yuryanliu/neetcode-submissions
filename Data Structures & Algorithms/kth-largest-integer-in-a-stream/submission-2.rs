use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    k : i32,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut min_heap = BinaryHeap::new();
        for num in nums {
            KthLargest::op(&mut min_heap, num, k);
        }
        Self {
            k,
            min_heap,
        }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        KthLargest::op(&mut self.min_heap, val, self.k);
        self.min_heap.peek().unwrap().0
    }

    fn op(min_heap: &mut BinaryHeap<Reverse<i32>>, num: i32, k: i32) {
        if min_heap.len() < k as usize {
            min_heap.push(Reverse(num));
        } else if let Some(top) = min_heap.peek() && top.0 < num  {
            min_heap.pop();
            min_heap.push(Reverse(num));
        }
    }
}
