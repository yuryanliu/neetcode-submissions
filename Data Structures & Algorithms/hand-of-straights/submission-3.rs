impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let n = hand.len() as i32;
        if n % group_size != 0 {
            return false;
        }
        let mut map = BTreeMap::<i32,i32>::new();
        for num in hand {
            *map.entry(num).or_default() += 1;
        }
        for i in 0..n/group_size {
            if let Some((&first_num, &count)) = map.first_key_value() {
                if count == 1 {
                    map.remove(&first_num);
                } else {
                    if let Some(count) = map.get_mut(&first_num) {
                        *count -= 1;
                    }
                }
                for j in 1..group_size {
                    let next_num = first_num+j;
                    if let Some(count) = map.get_mut(&next_num) {
                        *count -= 1;
                        if *count == 0 {
                            map.remove(&next_num);
                        }
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}
