// https://www.geeksforgeeks.org/doubly-linked-list/
use std::cell::RefCell;
use std::rc::Rc;

struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let node = Rc::new(RefCell::new(Node {
            data,
            next: self.head.take(),
            prev: None,
        }));

        self.head = Some(node)
    }

    pub fn push_back(&mut self, data: T) {
        let node = Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: self.head.take(),
        }));

        self.tail = Some(node)
    }

    pub fn push_at_index() {}

    pub fn pop_front(&mut self) -> Option<T>
    where
        T: Clone,
    {
        let value = self.head.clone().unwrap().borrow().data.clone();

        self.head = self.head.unwrap().borrow().next.clone();

        Some(value)
    }
}
