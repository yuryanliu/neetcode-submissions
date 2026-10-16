impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        if arr.len() <= 1 {
            return arr.len() as i32;
        }
        let mut diffs = vec![0; arr.len()];
        for i in 1..arr.len() {
            let diff = arr[i] - arr[i-1];
            if diff > 0 {
                diffs[i] = 1;
            } else if diff < 0 {
                diffs[i] = -1;
            }
        }
        let (mut begin, mut end, mut global) = (0, 0, 1);
        while end < diffs.len() {
            if diffs[end] == 0 {
                global = global.max((end - begin) as i32);
                begin = end;
            } else if diffs[end] == diffs[end-1]  {
                global = global.max((end - begin) as i32);
                begin = end - 1;
            }
            end += 1;
        }
        global = global.max((end - begin) as i32);
        global
    }
}
