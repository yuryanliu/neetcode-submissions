use std::collections::BTreeMap;
impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut m = BTreeMap::<i32,i32>::new();
        for trip in trips {
            *m.entry(trip[1]).or_default() += trip[0];
            *m.entry(trip[2]).or_default() -= trip[0];
        }
        let (mut global, mut local) = (0, 0);
        for &passengers in m.values() {
            local += passengers;
            global = global.max(local);
            if global > capacity {
                return false;
            }
        }
        true
    }
}
