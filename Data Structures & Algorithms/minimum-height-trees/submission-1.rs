impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        assert!(n>0);
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        let mut degrees = vec![0; n];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
            degrees[edge[0] as usize]+= 1;
            degrees[edge[1] as usize]+= 1;
        }
        let mut visited = vec![false; n];
        let mut count = n;
        while count > 2 {
            let mut removed = vec![];
            for i in 0..degrees.len() {
                if degrees[i] == 1 {
                    removed.push(i);
                }
            }
            for i in removed {
                visited[i] = true;
                degrees[i] -= 1;
                count -= 1;
                for &j in &graph[i] {
                    degrees[j] -= 1;
                }
            }
        }
        let mut res = vec![];
        for i in 0..visited.len() {
            if !visited[i] {
                res.push(i as i32);
            }
        }
        res
    }
}
