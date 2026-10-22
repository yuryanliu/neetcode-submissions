impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        // Priorty Queue: O(nlogk)
        Solution::find_kth_largest_min_heap(nums, k)
        // Quick Selection: O(n) 
    }

    fn find_kth_largest_min_heap(nums: Vec<i32>, k: i32) -> i32 {
        let mut min_heap = BinaryHeap::<Reverse<i32>>::new();
        for num in nums {
            min_heap.push(Reverse(num));
            if min_heap.len() as i32 > k {
                min_heap.pop();
            }
        }
        min_heap.pop().unwrap().0
    }

    //fn find_kth_largest_quick_selection(nums: Vec<i32>, k: i32) -> i32 {
    //    0
    //}
}
