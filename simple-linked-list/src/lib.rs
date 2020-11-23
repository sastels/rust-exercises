use std::iter::FromIterator;
use std::mem;

pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}

impl<T: Copy> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut node = &self.head;
        loop {
            match node {
                None => return len,
                Some(boxed_node) => {
                    node = &boxed_node.next;
                    len = len + 1;
                }
            }
        }
    }

    pub fn push(&mut self, element: T) {
        let mut new_node: Node<T> = Node {
            data: element,
            next: None,
        };
        match &self.head {
            Some(_) => new_node.next = mem::replace(&mut self.head, None),
            None => {}
        }
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        match &self.head {
            None => return None,

            Some(boxed_data) => {
                let data = boxed_data.data;
                let new_node = mem::replace(&mut self.head, None);
                self.head = mem::replace(&mut new_node.unwrap().next, None);
                Some(data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => return None,
            Some(boxed_data) => Some(&boxed_data.data),
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut vec: Vec<T> = self.into();
        vec.reverse();

        let mut new_list: SimpleLinkedList<T> = SimpleLinkedList::new();
        for x in vec {
            new_list.push(x);
        }
        new_list
    }
}

impl<T: Copy> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut new_list: SimpleLinkedList<T> = SimpleLinkedList::new();
        for x in iter {
            new_list.push(x);
        }
        new_list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T: Copy> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec: Vec<T> = vec![];
        if self.head.is_none() {
            return vec;
        }

        let mut node = self.head.unwrap();
        loop {
            vec.push(node.data);
            match node.next {
                None => {
                    vec.reverse();
                    return vec;
                }
                Some(bn) => node = bn,
            }
        }
    }
}
