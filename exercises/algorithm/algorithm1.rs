/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}

struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T: Ord + Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self {
        let mut merged_list = LinkedList::new();

        let mut current_a = list_a.start;
        let mut current_b = list_b.start;

        while let (Some(node_a), Some(node_b)) = (current_a, current_b) {
            unsafe {
                if (*node_a.as_ptr()).val <= (*node_b.as_ptr()).val {
                    merged_list.add((*node_a.as_ptr()).val.clone());
                    current_a = (*node_a.as_ptr()).next;
                } else {
                    merged_list.add((*node_b.as_ptr()).val.clone());
                    current_b = (*node_b.as_ptr()).next;
                }
            }
        }

        while let Some(node_a) = current_a {
            unsafe {
                merged_list.add((*node_a.as_ptr()).val.clone());
                current_a = (*node_a.as_ptr()).next;
            }
        }

        while let Some(node_b) = current_b {
            unsafe {
                merged_list.add((*node_b.as_ptr()).val.clone());
                current_b = (*node_b.as_ptr()).next;
            }
        }

        merged_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.start;
        while let Some(node) = current {
            unsafe {
                write!(f, "{} ", node.as_ref().val)?;
                current = node.as_ref().next;
            }
        }
        Ok(())
    }
}
