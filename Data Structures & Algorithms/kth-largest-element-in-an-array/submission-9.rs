use rand::random;
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        // Priorty Queue: O(nlogk)
        //Solution::find_kth_largest_min_heap(nums, k)
        // Quick Selection: O(n) 
        Solution::find_kth_largest_quick_selection(&mut nums, k)
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

    fn find_kth_largest_quick_selection(nums:&mut [i32], k: i32) -> i32 {
        let (mut i, mut j, mut l) = (0i32, 0i32, nums.len() as i32 - 1);
        let pilot = nums[random::<usize>() % nums.len()];
        while j <= l {
            if nums[j as usize] > pilot {
                nums.swap(i as usize, j as usize);
                i += 1;
                j += 1;
            } else if nums[j as usize] == pilot {
                j += 1;
            } else {
                nums.swap(j as usize, l as usize);
                l -= 1;
            }
        }
        if k <= i {
            Solution::find_kth_largest_quick_selection(&mut nums[..i as usize], k)
        } else if k <= j {
            pilot
        } else {
            Solution::find_kth_largest_quick_selection(&mut nums[j as usize..], k-j)
        }
    }
}
