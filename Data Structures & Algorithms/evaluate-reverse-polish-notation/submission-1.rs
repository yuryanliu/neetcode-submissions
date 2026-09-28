impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for token in tokens {
            let n = if token == "+" || token == "-" ||
                token == "*" || token == "/" {
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                if token == "+" {
                    l + r
                } else if token == "-" {
                    l - r
                } else if token == "*" {
                    l * r
                } else /*if token == "/"*/ {
                    l / r
                }
            } else {
                token.parse::<i32>().unwrap()
            };
            stack.push(n)
        }
        stack.pop().unwrap()
    }
}
