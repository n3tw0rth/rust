use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    data: T,
    prev: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> DoublyLinkedList<T> {
    /// Create a new empty list
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    /// Insert at the front
    fn push_front(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            prev: None,
            next: self.head.clone(),
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                self.head = Some(new_node);
            }
            None => {
                // First node in the list
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }

    /// Insert at the back
    fn push_back(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            prev: self.tail.clone(),
            next: None,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            None => {
                // First node in the list
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    /// Remove from the front
    fn pop_front(&mut self) -> Option<T>
    where
        T: Clone,
    {
        self.head.take().map(|old_head| {
            let data = old_head.borrow().data.clone();
            self.head = old_head.borrow().next.clone();

            if let Some(new_head) = &self.head {
                new_head.borrow_mut().prev = None;
            } else {
                self.tail = None;
            }
            data
        })
    }

    /// Remove from the back
    fn pop_back(&mut self) -> Option<T>
    where
        T: Clone,
    {
        self.tail.take().map(|old_tail| {
            let data = old_tail.borrow().data.clone();
            self.tail = old_tail.borrow().prev.clone();

            if let Some(new_tail) = &self.tail {
                new_tail.borrow_mut().next = None;
            } else {
                self.head = None;
            }
            data
        })
    }

    /// Iterate forward
    fn iter_forward(&self) {
        let mut node = self.head.clone();
        while let Some(curr) = node {
            print!("{} -> ", curr.borrow().data);
            node = curr.borrow().next.clone();
        }
        println!("None");
    }

    /// Iterate backward
    fn iter_backward(&self) {
        let mut node = self.tail.clone();
        while let Some(curr) = node {
            print!("{} -> ", curr.borrow().data);
            node = curr.borrow().prev.clone();
        }
        println!("None");
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_back(3);
    list.push_back(4);

    println!("Forward:");
    list.iter_forward();

    println!("Backward:");
    list.iter_backward();

    list.pop_front();
    println!("After pop_front:");
    list.iter_forward();

    list.pop_back();
    println!("After pop_back:");
    list.iter_forward();
}
