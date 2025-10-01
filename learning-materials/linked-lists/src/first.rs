use std::{io::Empty, mem};

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let old_node = mem::replace(&mut self.head, Link::Empty);

        let new_node = Box::new(Node {
            elem: elem,
            next: old_node,
        });

        self.head = Link::More(new_node);
    }
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}
