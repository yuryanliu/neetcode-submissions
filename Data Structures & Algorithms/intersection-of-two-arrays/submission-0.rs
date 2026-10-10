impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let (mut set1, mut set2) = (HashSet::new(), HashSet::new());
        for num in nums1 {
            set1.insert(num);
        }
        for num in nums2 {
            set2.insert(num);
        }
        set1.intersection(&set2).map(|v| *v).collect()
    }
}
