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
        let mut queue: Queue<i32> = Queue::new();

        assert!(queue.is_empty());
        assert_eq!(queue.size(), 0);
        assert_eq!(queue.peek(), None);
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_one_element() {
        let mut queue: Queue<i32> = Queue::new();

        queue.enqueue(123);

        assert_eq!(queue.is_empty(), false);
        assert_eq!(queue.size(), 1);
        assert_eq!(queue.peek(), Some(&123));

        let result = queue.dequeue();

        assert_eq!(result, Some(123));
        assert!(queue.is_empty(), true);
        assert_eq!(queue.size(), 0);
        assert!(queue.peek().is_none());
        assert!(queue.dequeue().is_none());
    }

    #[test]
    fn test_two_element() {
        let mut queue: Queue<i32> = Queue::new();

        queue.enqueue(123);
        queue.enqueue(456);

        assert_eq!(queue.is_empty(), false);
        assert_eq!(queue.size(), 2);
        assert_eq!(queue.peek(), Some(&123));

        let result = queue.dequeue();

        assert_eq!(result, Some(123));
        assert_eq!(queue.is_empty(), false);
        assert_eq!(queue.size(), 1);
        assert_eq!(queue.peek(), Some(&456));

        let result = queue.dequeue();

        assert_eq!(result, Some(456));
        assert_eq!(queue.is_empty(), true);
        assert_eq!(queue.size(), 0);
        assert!(queue.peek().is_none());
        assert!(queue.dequeue().is_none());
    }

    #[test]
    fn test_iterator() {
        let mut queue: Queue<i32> = Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);
        queue.enqueue(5);
        queue.enqueue(6);

        let mut result: Vec<i32> = Vec::new();
        for element in queue {
            result.push(element);
        }

        assert_eq!(result, [1, 2, 3, 4, 5, 6]);
    }
}
