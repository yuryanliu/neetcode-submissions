impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        assert!(num_courses > 0);
        let mut visited = vec![false; num_courses as usize];
        let mut onstack = vec![false; num_courses as usize];
        let mut graph = vec![vec![]; num_courses as usize];
        for prerequisite in prerequisites {
            graph[prerequisite[1] as usize].push(prerequisite[0] as usize);
        }
        for i in 0..num_courses as usize {
            if !visited[i] {
                if Solution::dfs(&graph, &mut visited, &mut onstack, i) {
                    return false;
                }
            }
        }
        true
    }

    fn dfs(graph:&Vec<Vec<usize>>, visited:&mut Vec<bool>, onstack:&mut Vec<bool>, i: usize) -> bool {
        visited[i] = true;
        onstack[i] = true;

        for &j in &graph[i] {
            if onstack[j] {
                return true;
            } else if !visited[j] {
                if Solution::dfs(graph, visited, onstack, j) {
                    return true;
                }
            }
        }

        onstack[i] = false;
        false
    }
}
