impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        assert!(!nums.is_empty());
        let (lo, hi) = (0i32, nums.len() as i32 - 1);
        Solution::quick_select(&mut nums, k)
    }

    fn quick_select(nums:&mut [i32], k: i32) -> i32 {
        let (lo, hi) = (0, nums.len() as i32 - 1);
        let pilot = nums[lo as usize]; // random of [lo, hi] is better 
        let (mut i, mut j, mut l) = (lo, lo,  hi);
        while j <= l {
            if nums[j as usize] > pilot {
                nums.swap(i as usize, j as usize);
                i += 1;
                j += 1;
            } else if nums[j as usize] < pilot {
                nums.swap(j as usize, l as usize);
                l -= 1;
            } else {
                j += 1; 
            }
        }
        if k <= i {
            Solution::quick_select(&mut nums[lo as usize..=(i-1) as usize], k)
        } else if k <= j {
            pilot
        } else {
            Solution::quick_select(&mut nums[j as usize..=hi as usize], k - j)
        }
    }
}