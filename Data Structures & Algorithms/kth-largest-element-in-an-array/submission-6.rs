impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        assert!(!nums.is_empty());
        let (lo, hi) = (0i32, nums.len() as i32 - 1);
        Solution::quick_select(&mut nums, lo, hi, k)
    }

    fn quick_select(nums:&mut Vec<i32>, lo:i32, hi: i32, k: i32) -> i32 {
        if lo == hi {
            return nums[lo as usize];
        }
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
            Solution::quick_select(nums, lo, i - 1, k)
        } else if k <= j {
            pilot
        } else {
            Solution::quick_select(nums, j, hi, k)
        }
    }
}