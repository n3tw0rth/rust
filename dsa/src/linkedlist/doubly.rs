//! simple example
//!```
//!use dsa::linkedlist::DoublyLinkedList;
//!
//!fn main() {
//!    let mut doubly_linked_list: DoublyLinkedList<u8> = DoublyLinkedList::new();
//!
//!    println!("Is emtpy: {}", doubly_linked_list.is_emtpy());
//!
//!    doubly_linked_list.push_front(5);
//!    doubly_linked_list.push_front(4);
//!    doubly_linked_list.push_back(6);
//!
//!    doubly_linked_list.traverse_back();
//!    doubly_linked_list.traverse_front();
//!
//!    println!("Is emtpy: {}", doubly_linked_list.is_emtpy());
//!}
//!```

use std::cell::RefCell;
use std::fmt::Display;
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
            next: None,
            prev: None,
        }));
        if let Some(prev_head) = self.head.take() {
            prev_head.borrow_mut().prev = Some(Rc::clone(&node));
            node.borrow_mut().next = Some(prev_head);
            self.head = Some(node);
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
        }
    }

    pub fn push_back(&mut self, data: T) {
        let node = Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None,
        }));
        if let Some(prev_tail) = self.tail.take() {
            prev_tail.borrow_mut().next = Some(Rc::clone(&node));
            node.borrow_mut().prev = Some(prev_tail);
            self.tail = Some(node);
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
        }
    }

    pub fn is_emtpy(&self) -> bool {
        self.tail.is_none() | self.head.is_none()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            match head.borrow_mut().next.take() {
                Some(node) => {
                    node.borrow_mut().prev = None;
                    self.head = Some(node);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(head).ok().unwrap().into_inner().data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|tail| {
            match tail.borrow_mut().prev.take() {
                Some(node) => {
                    node.borrow_mut().next = None;
                    self.tail = Some(node);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(tail).ok().unwrap().into_inner().data
        })
    }

    pub fn traverse_back(&self)
    where
        T: Display,
    {
        let mut current = Some(Rc::clone(&self.tail.as_ref().unwrap()));

        while let Some(node) = current {
            let node_ref = node.borrow();
            println!("{}", node_ref.data);

            current = node_ref.prev.clone();
        }
    }

    pub fn traverse_front(&self)
    where
        T: Display,
    {
        let mut current = Some(Rc::clone(&self.head.as_ref().unwrap()));

        while let Some(node) = current {
            let node_ref = node.borrow();
            println!("{}", node_ref.data);

            current = node_ref.next.clone()
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::linkedlist::DoublyLinkedList;

    #[test]
    fn test_is_empty() {
        let mut linked_list: DoublyLinkedList<usize> = DoublyLinkedList::new();

        assert!(linked_list.is_emtpy());

        linked_list.push_front(5);
        assert!(!linked_list.is_emtpy());
    }

    #[test]
    fn test_pop_front() {
        let mut linked_list: DoublyLinkedList<usize> = DoublyLinkedList::new();

        linked_list.push_front(1);
        linked_list.push_front(2);

        assert_eq!(linked_list.pop_front(), Some(2))
    }

    #[test]
    fn test_pop_back() {
        let mut linked_list: DoublyLinkedList<usize> = DoublyLinkedList::new();

        linked_list.push_front(1);
        linked_list.push_front(2);

        assert_eq!(linked_list.pop_back(), Some(1))
    }
}
