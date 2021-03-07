pub struct Deque<T> {
    items: Vec<T>,
}

impl<T> Deque<T> {
    pub fn new() -> Deque<T> {
        Deque { items: Vec::new() }
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

    pub fn enqueue_front(&mut self, element: T) {
        self.items.insert(0, element);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn dequeue_front(&mut self) -> Option<T> {
        match self.is_empty() {
            false => Some(self.items.remove(0)),
            true => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.items.first()
    }
}

#[cfg(test)]
mod tests {
    use super::Deque;

    #[test]
    fn test_empty() {
        let mut deque: Deque<i32> = Deque::new();

        assert_eq!(deque.is_empty(), true);
        assert_eq!(deque.size(), 0);
        assert_eq!(deque.peek(), None);
        assert_eq!(deque.dequeue(), None);
        assert_eq!(deque.dequeue_front(), None);
        assert_eq!(deque.peek_front(), None);
    }

    #[test]
    fn test_one_element() {
        let mut deque: Deque<i32> = Deque::new();

        deque.enqueue(123);

        assert_eq!(deque.is_empty(), false);
        assert_eq!(deque.size(), 1);
        assert_eq!(deque.peek(), Some(&123));

        let result = deque.dequeue();

        assert_eq!(result, Some(123));
        assert_eq!(deque.is_empty(), true);
        assert_eq!(deque.size(), 0);
        assert_eq!(deque.peek(), None);
        assert_eq!(deque.dequeue(), None);
        assert_eq!(deque.dequeue_front(), None);
        assert_eq!(deque.peek_front(), None);
    }

    #[test]
    fn test_two_element() {
        let mut deque: Deque<i32> = Deque::new();

        deque.enqueue_front(543);
        deque.enqueue(123);
        deque.enqueue(456);

        assert_eq!(deque.is_empty(), false);
        assert_eq!(deque.size(), 3);
        assert_eq!(deque.peek_front(), Some(&543));

        let result = deque.dequeue_front();

        assert_eq!(result, Some(543));
        assert_eq!(deque.is_empty(), false);
        assert_eq!(deque.size(), 2);
        assert_eq!(deque.peek(), Some(&456));

        let result = deque.dequeue();

        assert_eq!(result, Some(456));
        assert_eq!(deque.is_empty(), false);
        assert_eq!(deque.size(), 1);
        assert_eq!(deque.peek(), Some(&123));

        let result = deque.dequeue();

        assert_eq!(result, Some(123));
        assert_eq!(deque.is_empty(), true);
        assert_eq!(deque.size(), 0);
        assert_eq!(deque.peek(), None);
        assert_eq!(deque.dequeue(), None);
        assert_eq!(deque.dequeue_front(), None);
        assert_eq!(deque.peek_front(), None);
    }
}
