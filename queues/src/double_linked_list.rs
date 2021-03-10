use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

struct Node<T> {
    element: T,
    next: Link<T>,
    prev: WeakLink<T>,
}

impl<T> Node<T> {
    pub fn new(element: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            element,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, element: T) {
        let new_head = Node::new(element);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_head));
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().element
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.element))
    }

    pub fn peek_front_mut(&self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.element))
    }
}

impl<T> List<T> {
    pub fn push_back(&mut self, element: T) {
        let new_tail = Node::new(element);

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                self.tail = Some(new_tail);
            }
            None => {
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|tail| {
            match tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.upgrade().map(|new_tail| {
                        new_tail.borrow_mut().next.take();
                        self.tail = Some(new_tail);
                    });
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(tail).ok().unwrap().into_inner().element
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.element))
    }

    pub fn peek_back_mut(&self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.element))
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

pub struct Iter<'a, T> {
    next: Option<Ref<'a, Node<T>>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|head| head.borrow()),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Ref<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        None
        // self.next.take().map(|node_ref| {
        //     let (next, element) = Ref::map_split(node_ref, |node| {
        //         (&node.next, &node.element)
        //     });
        //
        //     self.next = if next.is_some() {
        //         Some(Ref::map(next, |next| &**next.as_ref().unwrap()))
        //     } else {
        //         None
        //     };
        // self.next = next.as_ref().map(|node| node.borrow());
        // self.next = node_ref.next.as_ref().map(|node| node.borrow());
        // Ref::map(node_ref, |node| &node.element)
        // element
        // })

        // return None;
        // self.next.take().map(|node| {
        //
        // })
        // self.next.take().map(|node| {
        //     Ref::map_split(node, |node| {
        //
        //     })
        //     node.next.as_ref().map(|head| {
        //         self.next = Some(head.borrow())
        //     })
        //     // self.next = node.next;
        // });

        // return None;
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn front() {
        let mut list = List::new();

        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn back() {
        let mut list = List::new();

        assert_eq!(list.pop_back(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        list.push_back(4);
        list.push_back(5);

        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek_front() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(*list.peek_front().unwrap(), 3);
    }

    #[test]
    fn peek_front_mut() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        *list.peek_front_mut().unwrap() = 10;

        assert_eq!(*list.peek_front_mut().unwrap(), 10);
    }

    #[test]
    fn peek_back() {
        let mut list = List::new();

        assert!(list.peek_back().is_none());

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(*list.peek_back().unwrap(), 3);
    }

    #[test]
    fn peek_back_mut() {
        let mut list = List::new();

        assert!(list.peek_back().is_none());

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        *list.peek_back_mut().unwrap() += 10;

        assert_eq!(*list.peek_back().unwrap(), 13);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert!(iter.next_back().is_none());
        assert!(iter.next().is_none());
    }
}
