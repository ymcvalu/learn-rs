use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::{Rc, Weak};

pub struct Node<T: Sized + Debug> {
    value: T,
    prev: Option<Weak<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Sized + Debug> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            value: data,
            prev: None,
            next: None,
        }
    }
}

pub struct List<T: Sized + Debug> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Debug> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        match self.tail {
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
            Some(ref node) => {
                new_node.borrow_mut().prev = Some(Rc::downgrade(&node));
                node.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
        }
    }

    pub fn print_from_front(&self) {
        let mut node = self.head.clone();
        loop {
            match node.clone() {
                Some(ref r) => {
                    println!("{:?}", &r.deref().borrow().value);
                    node = r.deref().borrow().next.clone();
                }
                _ => break,
            }
        }
    }

    pub fn print_from_back(&self) {
        let mut node = self.tail.clone();
        loop {
            match node.clone() {
                Some(ref r) => {
                    println!("{:?}", &r.deref().borrow().value);

                    if let Some(ref wr) = r.deref().borrow().prev {
                        node = Weak::upgrade(wr);
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }
}
