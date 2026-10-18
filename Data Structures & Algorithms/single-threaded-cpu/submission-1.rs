impl Solution {
    pub fn get_order(mut tasks: Vec<Vec<i32>>) -> Vec<i32> {
        assert!(!tasks.is_empty());
        let (mut processing_heap, mut enqueue_heap) = (BinaryHeap::<Reverse<Vec<i64>>>::new(), BinaryHeap::<Reverse<Vec<i64>>>::new());
        for (index, task) in tasks.into_iter().enumerate() {
            enqueue_heap.push(Reverse(vec![task[0] as i64, task[1] as i64, index as i64]));
        }

        let (mut current_time, mut res) = (0i64, vec![]);
        while !processing_heap.is_empty() || !enqueue_heap.is_empty() {
            if processing_heap.is_empty() {
                if let Some(Reverse(task)) = enqueue_heap.peek() {
                    current_time = current_time.max(task[0]);
                }
            }
            while let Some(Reverse(task)) = enqueue_heap.peek() {
                if task[0] <= current_time {
                    processing_heap.push(Reverse(vec![task[1], task[2], task[0]]));
                    enqueue_heap.pop();
                } else {
                    break;
                }
            }
            if let Some(Reverse(curr_task)) = processing_heap.pop() {
                res.push(curr_task[1] as i32);
                current_time += curr_task[0];
            }
        }

        res
    }
}