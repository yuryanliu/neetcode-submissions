impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        assert!(num_courses > 0);
        let num_courses = num_courses as usize;
        let (mut graph, mut visited, mut onstack, mut postvisited) = (vec![], vec![false; num_courses], vec![false; num_courses], vec![]);
        for _ in 0..num_courses {
            graph.push(vec![]);
        }
        for prerequisite in prerequisites {
            graph[prerequisite[0] as usize].push(prerequisite[1] as usize);
        }
        for i in 0..num_courses {
            if !visited[i] {
                if Solution::dfs(&graph, &mut visited, &mut onstack, &mut postvisited, i) {
                    return vec![];
                }
            }
        }
        postvisited
    }

    fn dfs(graph:&Vec<Vec<usize>>, visited:&mut Vec<bool>, onstack:&mut Vec<bool>, postvisited:&mut Vec<i32>, i:usize) -> bool {
        visited[i] = true;
        //previsit
        onstack[i] = true;

        for &j in &graph[i] {
            if onstack[j] {
                return true;
            } else if !visited[j] {
                if Solution::dfs(graph, visited, onstack, postvisited, j) {
                    return true;
                }
            }
        }

        //postvisit
        postvisited.push(i as i32);
        onstack[i] = false;
        false
    }
}
