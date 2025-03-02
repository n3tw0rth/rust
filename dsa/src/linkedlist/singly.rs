//! A simple demonstration of using a linked list in Rust.
//!
//! # Example
//!
//! ```
//! use std::collections::LinkedList;
//!
//! fn main() {
//!     let mut linked_list: LinkedList<String> = LinkedList::new();
//!
//!     linked_list.push_back("a".to_string());
//!     linked_list.push_back("b".to_string());
//!
//!     let popped_value = linked_list.pop_front(); // Removes "a"
//!
//!     println!("{:?}", popped_value);
//!     println!("{:?}", linked_list.is_empty());
//! }
//! ```
//!
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, value: T) {
        let node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(node)
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take().unwrap();
        self.head = head.next;
        Some(head.value)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}
