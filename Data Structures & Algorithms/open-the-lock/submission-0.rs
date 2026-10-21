use std::collections::HashSet;
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let target = target.as_bytes().to_vec();
        let mut visited = HashSet::new();
        for deadend in deadends {
            visited.insert(deadend.as_bytes().to_vec());
        }
        let mut start = vec![b'0', b'0', b'0', b'0'];  
        if visited.contains(&start) {
            return -1;
        }
        let mut queue = VecDeque::new();
        visited.insert(start.clone());
        queue.push_back((start, 0));
        while let Some((start, turns)) = queue.pop_front() {
            if start == target {
                return turns;
            }

            for cand in Solution::construct_candidates(start) {
                if !visited.contains(&cand) {
                    visited.insert(cand.clone());
                    queue.push_back((cand, turns+1));
                }
            }
        }
        -1
    }

    fn construct_candidates(bytes: Vec<u8>) -> Vec<Vec<u8>> {
        let mut res = vec![];
        for i in 0..4 {
            let digit = (bytes[i] - b'0') as i32;
            let mut up = bytes.to_vec();
            up[i] = b'0' + ((digit + 1) % 10) as u8;
            res.push(up);
            let mut down = bytes.to_vec();
            down[i] = b'0' + ((digit + 9) % 10) as u8;
            res.push(down);
        }
        res
    }
}
