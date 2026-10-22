impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        assert!(num_courses > 0);
        let n = num_courses as usize;
        let (mut graph, mut visited) = (vec![vec![]; n], vec![false; n]);
        for prerequisity in prerequisites {
            graph[prerequisity[1] as usize].push(prerequisity[0] as usize);
        }

        let (mut onstack, mut postvisited) = (HashSet::new(), vec![]);
        for i in 0..n {
            if !visited[i] {
                if Solution::dfs(&graph, &mut visited, &mut onstack, &mut postvisited, i) {
                    return vec![];
                }
            }
        }
        postvisited.reverse();
        postvisited
    }

    fn dfs(graph:&Vec<Vec<usize>>, visited:&mut Vec<bool>, onstack:&mut HashSet<usize>, postvisited:&mut Vec<i32>, i: usize) -> bool {
        visited[i] = true;
        onstack.insert(i);

        for &j in &graph[i] {
            if onstack.contains(&j) {
                return true;
            }
            if !visited[j] {
                if Solution::dfs(graph, visited, onstack, postvisited, j) {
                    return true;
                }
            }
        }

        onstack.remove(&i);
        postvisited.push(i as i32);
        false
    }
}
