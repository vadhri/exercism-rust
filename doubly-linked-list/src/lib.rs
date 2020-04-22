// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;
use core::ptr::null_mut;
use core::marker::PhantomData;

struct Node<T> {
    elem: T,
    next: *mut Node<T>,
    prev: *mut Node<T>,
}

pub struct LinkedList<T> {
    dll_root: *mut Node<T>,
    dll_end: *mut Node<T>,
    length: usize
}

pub struct Cursor<'a, T> {
    dll: &'a mut LinkedList<T>,
    pointer: *mut Node<T>
}

pub struct Iter<'a, T> {
    lifetime: PhantomData<&'a T>,
    start: *mut Node<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            dll_root: null_mut(),
            dll_end: null_mut(),
            length: 0
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            pointer: self.dll_root,
            dll: self
        }
    }

    pub fn cursor_back(&mut self) -> Cursor<T> {
        Cursor {
            pointer: self.dll_end,
            dll: self
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            start: self.dll_root,
            lifetime: PhantomData
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        loop {
            if let None = self.cursor_front().take() {
                break;
            }
        }
    }
}

impl<T> Cursor<'_, T> {
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            if self.pointer.is_null() {
                 None
            } else {
                Some(&mut (*self.pointer).elem)
            }
        }
    }

    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            if (*self.pointer).next.is_null() {
                None
            } else {
                let element = &mut (*(*self.pointer).next).elem;
                self.pointer = (*self.pointer).next;
                Some(element)
            }
        }
    }

    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            if (*self.pointer).prev.is_null() {
                None
            } else {
                let element = &mut (*(*self.pointer).prev).elem;
                self.pointer = (*self.pointer).prev;
                Some(element)
            }
        }
    }

    pub fn take(&mut self) -> Option<T> {
        unsafe {
            // no elements in the list //
            if self.pointer.is_null() {
                None
            } else {
                self.dll.length -= 1;

                let node = Box::from_raw(self.pointer);
                if node.prev.is_null() && node.next.is_null() {
                    self.pointer = null_mut();
                    self.dll.dll_root = null_mut();
                    self.dll.dll_end = null_mut();
                } else if !node.prev.is_null() && node.next.is_null()  {
                    // last element in the list
                    self.pointer = node.prev;
                    self.dll.dll_end = node.prev;
                    (*node.prev).next = null_mut();
                } else if node.prev.is_null() && !node.next.is_null() {
                    // first element in the list //
                    self.pointer = node.next;
                    (*node.next).prev = null_mut();
                    self.dll.dll_root = node.next;
                } else if !node.prev.is_null() && !node.next.is_null() {
                    // middle element //
                    self.pointer = node.next;
                    (*node.next).prev = node.prev;
                    (*node.prev).next = node.next;
                }

                Some(node.elem)
            }
        }
    }

    pub fn insert_after(&mut self, element: T) {
        unsafe {
            // first time node is null ptr.
            let new_node = Box::into_raw(Box::new(
                Node {
                    prev: null_mut(),
                    next: null_mut(),
                    elem: element
                }
            ));

            if self.pointer.is_null() {
                self.pointer = new_node;
                self.dll.dll_root = new_node;
                self.dll.dll_end = new_node;
            } else {
                (*new_node).prev = self.pointer;
                if !(*self.pointer).next.is_null() {
                    (*(*self.pointer).next).prev = new_node;
                }
                (*new_node).next = (*self.pointer).next;
                (*self.pointer).next = new_node;
                self.dll.dll_end = new_node;
            }
            self.dll.length += 1;
        }
    }

    pub fn insert_before(&mut self, element: T) {
        unsafe {
            // first time node is null ptr.
            let new_node = Box::into_raw(Box::new(
                Node {
                    prev: null_mut(),
                    next: null_mut(),
                    elem: element
                }
            ));

            if self.pointer.is_null() {
                // empty list
                self.dll.dll_root = new_node;
                self.dll.dll_end = new_node;
            } else {
                if (*self.pointer).prev.is_null() {
                    // first element in the list.
                    (*new_node).next = self.pointer;
                    (*new_node).prev = (*self.pointer).prev;

                    (*self.pointer).prev = new_node;
                    self.pointer = new_node;
                    self.dll.dll_root = new_node;
                } else {
                    // mid list and end element
                    (*(*self.pointer).prev).next  = new_node;
                    (*new_node).prev = (*self.pointer).prev;

                    (*self.pointer).prev = new_node;
                    (*new_node).next = self.pointer;
                }
            }
            self.dll.length += 1;
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unsafe {
            if self.start.is_null() {
                None
            } else {
                let element = &(*self.start).elem;
                self.start = (*self.start).next;
                Some(element)
            }
        }
    }
}
