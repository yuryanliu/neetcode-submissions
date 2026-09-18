/**
 * Definition of Interval:
 * #[derive(Debug, Clone)]
 * pub struct Interval {
 *     pub start: i32,
 *     pub end: i32,
 * }
 *
 * impl Interval {
 *     pub fn new(start: i32, end: i32) -> Self {
 *         Interval { start, end }
 *     }
 * }
 */

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Interval>) -> i32 {
        let mut m = BTreeMap::new();
        for interval in intervals {
            m.entry(interval.start).and_modify(|count| *count += 1).or_insert(1);
            m.entry(interval.end).and_modify(|count| *count -= 1).or_insert(-1);
        }
        let (mut global, mut local) = (0, 0);
        for (key, count) in m {
            local += count;
            global = global.max(local);
        }
        global
    }
}
