pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn push(&mut self, element: T) {
        self.items.push(element);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}

impl<T> Iterator for Stack<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn test_empty() {
        let mut stack: Stack<i32> = Stack::new();

        assert_eq!(stack.is_empty(), true);
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_one_element() {
        let mut stack: Stack<i32> = Stack::new();

        stack.push(123);

        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.peek(), Some(&123));

        let result = stack.pop();

        assert_eq!(result, Some(123));
        assert_eq!(stack.is_empty(), true);
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_two_element() {
        let mut stack: Stack<i32> = Stack::new();

        stack.push(123);
        stack.push(456);

        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.size(), 2);
        assert_eq!(stack.peek(), Some(&456));

        let result = stack.pop();

        assert_eq!(result, Some(456));
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.peek(), Some(&123));

        let result = stack.pop().unwrap();

        assert_eq!(result, 123);
        assert_eq!(stack.is_empty(), true);
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_iterator() {
        let mut stack: Stack<i32> = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);
        stack.push(6);

        let mut result: Vec<i32> = Vec::new();
        for element in stack {
            result.push(element);
        }

        assert_eq!(result, [6, 5, 4, 3, 2 ,1]);
    }
}
