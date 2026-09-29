use std::collections::HashMap;

struct TimeMap {
    store: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.store.entry(key).or_default().push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(values) = self.store.get(&key) {
            let (mut lo, mut hi) = (0i32, values.len() as i32 - 1);
            while lo <= hi {
                let mi = lo + (hi - lo) / 2;
                if values[mi as usize].0 == timestamp {
                    return values[mi as usize].1.clone();
                } else if values[mi as usize].0 < timestamp {
                    lo = mi + 1;
                } else {
                    hi = mi - 1;
                }
            }
            if hi < 0 {
                String::new()
            } else {
                values[hi as usize].1.clone()
            }
        } else {
            String::new()
        }
    }
}
