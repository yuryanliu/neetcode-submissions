struct StockSpan {
    price: i32,
    span: i32,
}
struct StockSpanner {
    monotonic_stack: Vec<StockSpan>,
}

impl StockSpanner {
    pub fn new() -> Self {
        Self {
            monotonic_stack: vec![],
        }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        while let Some(top) = self.monotonic_stack.last() && top.price <= price {
            span += top.span;
            self.monotonic_stack.pop();
        }
        self.monotonic_stack.push(StockSpan{
            price, 
            span,
        });
        span
    }
}
