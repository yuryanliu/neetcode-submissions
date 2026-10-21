#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Account {
    name: String,
    email: String,
}
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut graph: HashMap<Account, Vec<Account>> = HashMap::new();
        for account in accounts {
            let name = account[0].clone();
            if account.len() == 2 {
                graph.entry(Account {
                    name: name.clone(),
                    email: account[1].clone(),
                }).or_default();
            } else {
                for i in 1..account.len() {
                    for j in i+1..account.len() {
                        graph.entry(Account {
                            name: name.clone(),
                            email: account[i].clone(),
                        }).or_default().push(Account {
                            name: name.clone(),
                            email: account[j].clone()
                        });

                        graph.entry(Account {
                            name: name.clone(),
                            email: account[j].clone(),
                        }).or_default().push(Account {
                            name: name.clone(),
                            email: account[i].clone(),
                        });
                    }
                }
            }
        }

        let mut visited = HashSet::new();

        let mut res = vec![];
        for account in graph.keys() {
            if !visited.contains(account) {
                let mut cc = vec![account.name.clone()];
                Solution::dfs(&graph, &mut visited, &mut cc, account);
                res.push(cc);
            }
        }
        res
    }

    fn dfs(graph:&HashMap<Account, Vec<Account>>, visited:&mut HashSet<Account>, 
           cc:&mut Vec<String>, account: &Account) {
        visited.insert(account.clone());
        cc.push(account.email.clone());

        if let Some(neighbors) = graph.get(account) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    Solution::dfs(graph, visited, cc, neighbor);
                }
            }
        }
    }
}
