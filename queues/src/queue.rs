pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { items: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn enqueue(&mut self, element: T) {
        self.items.push(element);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        match self.is_empty() {
            false => Some(self.items.remove(0)),
            true => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.first()
    }
}

impl<T> Iterator for Queue<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.dequeue()
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_empty() {
        let mut stack: Queue<i32> = Queue::new();

        assert_eq!(stack.is_empty(), true);
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.dequeue(), None);
    }

    #[test]
    fn test_one_element() {
        let mut stack: Queue<i32> = Queue::new();

        stack.enqueue(123);

        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.peek(), Some(&123));

        let result = stack.dequeue();

        assert_eq!(result, Some(123));
        assert_eq!(stack.is_empty(), true);
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.dequeue(), None);
    }

    #[test]
    fn test_two_element() {
        let mut stack: Queue<i32> = Queue::new();

        stack.enqueue(123);
        stack.enqueue(456);

        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.size(), 2);
        assert_eq!(stack.peek(), Some(&123));

        let result = stack.dequeue();

        assert_eq!(result, Some(123));
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.peek(), Some(&456));

        let result = stack.dequeue();

        assert_eq!(result, Some(456));
        assert_eq!(stack.is_empty(), true);
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.dequeue(), None);
    }

    #[test]
    fn test_iterator() {
        let mut stack: Queue<i32> = Queue::new();

        stack.enqueue(1);
        stack.enqueue(2);
        stack.enqueue(3);
        stack.enqueue(4);
        stack.enqueue(5);
        stack.enqueue(6);

        let mut result: Vec<i32> = Vec::new();
        for element in stack {
            result.push(element);
        }

        assert_eq!(result, [1, 2, 3, 4, 5, 6]);
    }
}
