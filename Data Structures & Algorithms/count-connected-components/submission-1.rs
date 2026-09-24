impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        assert!(n > 0);
        let mut visited = vec![false; n as usize];
        let mut graph = vec![vec![]; n as usize];
        for edge in edges {
            graph[edge[1] as usize].push(edge[0] as usize);
            graph[edge[0] as usize].push(edge[1] as usize);
        }
        let mut cc = 0;
        for i in 0..n as usize {
            if !visited[i] {
                Solution::dfs(&graph, &mut visited, i);
                cc += 1;
            }
        }
        cc
    }

    fn dfs(graph:&Vec<Vec<usize>>, visited:&mut Vec<bool>, i: usize) {
        visited[i] = true;
        for &j in &graph[i] {
            if !visited[j]{
                Solution::dfs(graph, visited, j);
            }
        }
    }
}
