impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let mut found = [false; 3];
        for t in &triplets {
            if t[0] <= target[0] && t[1] <= target[1] && t[2] <= target[2] {
                if t[0] == target[0] {
                    found[0] = true;
                }
                if t[1] == target[1] {
                    found[1] = true;
                }
                if t[2] == target[2] {
                    found[2] = true;
                }
            }
        }
        found[0] && found[1] && found[2]
    }
}