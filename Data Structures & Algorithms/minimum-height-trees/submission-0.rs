impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        assert!(n>0);
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
        }
        let mut heights = vec![0; n];
        let mut min_height = i32::MAX;
        for i in 0..n {
            heights[i] = Solution::height(&graph, i);
            min_height = min_height.min(heights[i]);
        }
        heights.into_iter().enumerate().filter(|(i, h)| *h == min_height).map(|(i, _)| i as i32).collect()
    }

    fn height(graph:&Vec<Vec<usize>>, i: usize) -> i32 {
        let mut dists = vec![i32::MAX; graph.len()];
        let mut queue = VecDeque::new();
        let mut height = 0;
        dists[i] = 0;
        queue.push_back(i);
        while let Some(i) = queue.pop_front() {
            height = height.max(dists[i]);
            for &j in &graph[i] {
                if dists[j] > dists[i] + 1 {
                    dists[j] = dists[i] + 1;
                    queue.push_back(j);
                }
            }
        }
        height
    }
}
