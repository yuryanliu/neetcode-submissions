impl Solution {
    pub fn merge_triplets(mut triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        triplets.retain(|triplet| triplet[0]<=target[0] && triplet[1]<=target[1] && triplet[2]<=target[2]);
        if triplets.is_empty() {
            return false;
        }
        let mut max_triplet = triplets[0].clone();
        for triplet in triplets.into_iter().skip(1) {
            for i in 0..3 {
                max_triplet[i] = max_triplet[i].max(triplet[i]);
            }
        }
        max_triplet == target
    }
}
