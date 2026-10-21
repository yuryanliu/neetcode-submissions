impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        assert!(n>0);
        let (n, src, dst) = (n as usize, src as usize, dst as usize);
        let mut graph = vec![vec![]; n];
        for flight in flights {
            graph[flight[0] as usize].push((flight[1] as usize, flight[2]));
        }
        let (mut dists, mut prices) = (vec![i32::MAX; n], vec![i32::MAX; n]);
        let mut queue = VecDeque::new();
        dists[src] = 0; prices[src] = 0;
        queue.push_back((src, 0, 0));
        while let Some((curr, cost, hop)) = queue.pop_front() {
            if hop > k {
                continue;
            }
            for next in &graph[curr] {
                if prices[next.0] > cost + next.1 {
                    prices[next.0] = cost + next.1;
                    queue.push_back((next.0, cost + next.1, hop+1));
                }
            }
        }
        if prices[dst] != i32::MAX {
            prices[dst]
        } else {
            -1
        }
    }
}
