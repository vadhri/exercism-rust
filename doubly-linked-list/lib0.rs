// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;
use core::fmt::Debug;
use std::borrow::BorrowMut;
use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(PartialEq, Debug, Clone)]
struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

pub struct Cursor<'lifetime, T> {
    pos: &'lifetime mut Link<T>
}

pub struct Iter<'a, T>(std::marker::PhantomData<&'a T>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, tail: None }
    }

    pub fn len(&self) -> usize {
        let mut temp = self.head.clone();
        let mut count = 0;

        while !temp.is_none() {
            temp = temp.unwrap().borrow().next.clone();
            count += 1;
        }
        println!("len {:?}", count);
        count as usize
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<T> {
        println!("cursor_front .. ");

        Cursor {
            pos: &mut None
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<T> {
        let mut position: Link<T> = None;
        println!("cursor back .. ");
        let mut temp = self.head;
        let mut count = 0;

        while !temp.is_none() {
            println!("count -> {:?}", count);
            temp = temp.unwrap().borrow().next.clone();
            count += 1;
        }

        Cursor {
            pos: &mut temp
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        unimplemented!()
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<T> where T: Debug {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unimplemented!()
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    pub fn next(&mut self) -> Option<&mut T> {
        unimplemented!()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unimplemented!()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        unimplemented!()
    }

    pub fn insert_after(&mut self, _element: T) {
        println!("insert after {:?}", _element);
        let mut new_node = Node::new(_element);
        let mut position: Link<T> = None;

        match &self.pos {
            Some(value) => {
                println!("Prev value -> {:?}", value.borrow().prev);
                position = Some(Rc::clone(&value));
            }, None => {
                println!("No value inserted so far.");
                self.pos = Some(new_node);
            }
        }
    }

    pub fn insert_before(&mut self, _element: T) {
        unimplemented!()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unimplemented!()
    }
}
