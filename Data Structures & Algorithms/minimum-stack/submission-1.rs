struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<(i32, usize)>,
}

impl MinStack {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            min_stack: vec![],
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_stack.is_empty() {
            self.min_stack.push((val, self.stack.len()));
        } else if let Some(top) = self.min_stack.last() &&
            val < top.0 {
            self.min_stack.push((val, self.stack.len()));
        }
    }

    pub fn pop(&mut self) {
        let len = self.stack.len();
        if let Some(stack_top) = self.stack.pop() {
            if let Some(top) = self.min_stack.last() &&
                stack_top == top.0 && len == top.1 {
                self.min_stack.pop();
            }
        }
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        self.min_stack.last().unwrap().0
    }
}
