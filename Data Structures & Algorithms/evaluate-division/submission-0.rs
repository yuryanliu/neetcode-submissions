use std::collections::HashMap;

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut graph:HashMap<String, Vec<(String, f64)>> = HashMap::new();
        for (equation, value) in equations.into_iter().zip(values.into_iter()) {
            graph.entry(equation[0].clone()).or_default().push((equation[1].clone(), value));
            graph.entry(equation[1].clone()).or_default().push((equation[0].clone(), 1.0f64/value));
        }
        let mut res = vec![];
        for query in queries {
            if graph.contains_key(&query[0]) && graph.contains_key(&query[1]) {
                res.push(Solution::bfs(&graph, query));
            } else {
                res.push(-1.0f64);
            }
        }
        res
    }

    fn bfs(graph:&HashMap<String, Vec<(String, f64)>>, query: Vec<String>) -> f64 {
        let (mut queue, mut visited) = (VecDeque::new(), HashSet::new());
        queue.push_back((query[0].clone(), 1.0f64));
        visited.insert(query[0].clone());
        while let Some((s, m)) = queue.pop_front() {
            if s == query[1] {
                return m;
            }
            if let Some(neighbors) = graph.get(&s) {
                for neighbor in neighbors {
                    if !visited.contains(&neighbor.0) {
                        queue.push_back((neighbor.0.clone(), m * neighbor.1));
                        visited.insert(neighbor.0.clone());
                    }
                }
            }
        }
        -1.0f64
    }
}
