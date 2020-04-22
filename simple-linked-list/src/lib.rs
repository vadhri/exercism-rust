use std::iter::FromIterator;

#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> SimpleLinkedList<T> where T: PartialOrd + Clone {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None
        }
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        let mut head = self.head.as_ref();

        while let Some(node) = head {
            length += 1;
            head = node.next.as_ref();
        };
        length as usize
    }

    pub fn push(&mut self, _element: T) {
        let head = self.head.take();
        self.head = Some(Box::new(Node {
            data: _element,
            next: head
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().and_then(|node| {
            Some(&node.data)
        })
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut head = self.head.as_ref();
        let mut rev_list = SimpleLinkedList::new();

        while let Some(node) = head {
            let _element_data = node.data.clone();
            rev_list.push(_element_data);
            head = node.next.as_ref();
        };

        rev_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> where T: PartialOrd + Clone {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();

        for i in _iter {
            list.push(i);
        }

        list
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

impl<T> Into<Vec<T>> for SimpleLinkedList<T> where T: PartialOrd + Clone {
    fn into(self) -> Vec<T> {
        let mut vector = Vec::new();
        let mut head = self.head.as_ref(); 

        while let Some(node) = head {
            vector.push(node.data.clone());
            head = node.next.as_ref();
        };

        vector.reverse();
        vector
    }
}
