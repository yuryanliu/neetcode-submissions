impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack:Vec<i32> = vec![];
        for token in tokens {
            if token == "+" || token == "-" || token == "*" || token == "/" {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let c = if token == "+" {
                    a + b
                } else if token == "-" {
                    a - b
                } else if token == "*" {
                    a * b
                } else {//token == "/"
                    a / b
                };
                stack.push(c);
            } else {
                stack.push(token.parse::<i32>().unwrap());
            }
        }
        stack[0]
    }
}
