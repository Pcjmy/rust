struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {

    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_stack.is_empty() || val <= *self.min_stack.last().unwrap() {
            self.min_stack.push(val);
        }
    }

    fn pop(&mut self) {
        if let Some(val) = self.stack.pop() {
            if !self.min_stack.is_empty() && *self.min_stack.last().unwrap() == val {
                self.min_stack.pop();
            }
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

fn main() {
    let mut min_stack = MinStack::new();

    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);

    println!("getMin: {}", min_stack.get_min());  // Should print -3

    min_stack.pop();

    println!("top: {}", min_stack.top());        // Should print 0
    println!("getMin: {}", min_stack.get_min()); // Should print -2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_stack() {
        let mut min_stack = MinStack::new();

        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);

        assert_eq!(min_stack.get_min(), -3);

        min_stack.pop();

        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}