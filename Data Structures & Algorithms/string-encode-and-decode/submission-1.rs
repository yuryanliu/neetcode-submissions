impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        // vec_len#str1_len#....str2_len#...
        let mut res = format!("{}#", strs.len());
        for s in strs {
            res += format!("{}#{}", s.len(), s).as_str();
        }
        res
    }

    pub fn decode(s: String) -> Vec<String> {
        let (l, mut remain) = s.split_once('#').unwrap();
        let vec_len = l.parse().unwrap();
        let mut res = vec![];
        for i in 0..vec_len {
            let (l, r) = remain.split_once('#').unwrap();
            let (l, r) = r.split_at(l.parse().unwrap());
            remain = r;
            res.push(l.to_string());
        }
        res
    }
}
