impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let n = hand.len();
        let k = group_size as usize;
        if n % k != 0 {
            return false;
        }

        let mut counts = BTreeMap::new();
        for &card in &hand {
            *counts.entry(card).or_insert(0) += 1;
        }

        while let Some((&card, &count)) = counts.iter().next() {
            if count == 0 {
                counts.remove(&card);
                continue;
            }
            for i in 0..group_size {
                let next_card = card + i;
                let entry = counts.entry(next_card).or_insert(0);
                if *entry < count {
                    return false;
                }
                *entry -= count;
            }
        }

        true
    }
}