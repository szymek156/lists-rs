use std::{fmt::Debug, ops::Deref};

#[derive(PartialEq, Debug)]
pub struct List<T> {
    head: Link<T>,
}
#[derive(PartialEq, Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}
type Link<T> = Option<Box<Node<T>>>;

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.head.as_ref().map(|elem| &elem.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn push(&mut self, elem: T) {
        // head -> Node
        // new_node -> Node, head -> Empty
        let new_node = Box::new(Node {
            elem: elem,

            // Takes the value out of the option, leaving a None in its place.
            next: self.head.take(), // mem::replace(&mut self.head, None),
        });

        // cannot write:
        // let new_node = Box::new(Node{
        //     elem: elem,
        //     next: self.head
        // });

        // Because that moves self.head leaving self in a bad state
        // Even, later we do set self.head to some meaningful value
        // Rust compiler still disallows that, because of exception safety
        // Hence we use mem::replace which moves value from head, and returns it, while
        // at the same time we put something in it (Empty in this case)

        // head -> new_node -> Node
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn push_back(&mut self, elem: T) {
        let link = Some(Box::new(Node {
            elem: elem,
            next: None,
        }));

        let mut tail = &mut self.head;

        loop {
            match tail {
                None => break,
                Some(more) => tail = &mut more.next,
            }
        }

        *tail = link;
    }
}

// Version 1, create a dedicated struct for IntoIterator
// pub struct IntoIter<T>(List<T>);

// impl<T> Iterator for IntoIter<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.pop()
//     }
// }

// impl<T> IntoIterator for List<T> {
//     type Item = T;

//     type IntoIter = IntoIter<T>;

//     fn into_iter(self) -> Self::IntoIter {
//         IntoIter(self)
//     }
// }

// Version 2 Implement Iterator directly for list
// Single responsibility principle up your butt!
impl<T> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

// There is a blank trait implementation in std::core:
// impl<I> IntoIterator for I
// where I: Iterator;
// Anything what implements Iterator, implements IntoIterator

// pub struct Iter<'a, T> {
//     next: Option<&'a Node<T>>,
// }

// impl<T> List<T> {
//     // Compiler deduces lifetime for Iter is the same as for self,
//     // Makes sense -> Iter needs to lives as long as &self (List)
//     pub fn iter(&self) -> Iter<T> {
//         Iter {

//             // node: &Box<Node<T>>
//             // *node: Box<Node<T>>
//             // **node: Node<T>
//             // &**node: &Node<T>
//             // next: self.head.as_ref().map(|node| &**node),

//             // node.as_ref() calls method from Box, which is clever enough to figure we mean
//             // &Node<T>, not &Box<Node<T>>

//             next: self.head.as_ref().map(|node| {let t= node.as_ref(); t}),
//         }
//     }
// }

// impl<'a, T> Iterator for Iter<'a, T> {
//     type Item = &'a T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.map(|node| {
//             self.next = node.next.as_ref().map(|node| node.as_ref());
//             &node.elem
//         })
//     }
// }

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        // As deref for Option extracts value from some:
        // Some(a) -> a.deref()
        // where 'a' is Box<Node<T>>
        // calling deref on a box results with reference to underlying value -> &Node<T>
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // map takes self by value, but since Item is an immutable reference
        // there is no move but COPY (you can have as many imm references as you like)
        // 
        // self.next.map(|node| { &node.elem });
        //
        // It's possible to use self.next, after map:
        // println!("self next after {:?}", self.next);
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        // As deref for Option extracts value from some:
        // Some(a) -> a.deref()
        // where 'a' is Box<Node<T>>
        // calling deref on a box results with reference to underlying value -> &Node<T>
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // self.next.take(), sets next to None, and we get control of value from next
        self.next.take().map(|node| {
            // We map Some(a) to a

            // We set next to point to another element
            self.next = node.next.as_deref_mut();

            // And we return current element
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // Take ownership over the head, sets head to None
        let mut elem = self.head.take();

        while let Some(boxed) = elem {
            // Old value got drop here
            elem = boxed.next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_back_works() {
        let mut list = List::new();

        assert_eq!(list, List { head: None });

        list.push_back(69);

        assert_eq!(
            list,
            List {
                head: Some(Box::new(Node {
                    elem: 69,
                    next: None
                }))
            }
        );

        list.push_back(13);

        assert_eq!(
            list,
            List {
                head: Some(Box::new(Node {
                    elem: 69,
                    next: Some(Box::new(Node {
                        elem: 13,
                        next: None
                    }))
                }))
            }
        );
    }

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn break_the_stack() {
        {
            let mut list = List::new();

            for i in 1..100000 {
                list.push(i);
            }
            println!("Leaving, call dtor");
            // Without custom Drop implementation, compiler does:
            // List.drop -> Link.drop() -> Box.drop() -> Node.drop() -> next.drop() (Link.drop())
            //                ^---------------------------------------------------------|
            // Recursive call! Tail recursion cannot be applied here
            // Having list with huge amount of elements will blow up the stack
        }

        println!("Still alive!");
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        // |value| is of type &mut i32
        // |&mut value| is a pattern matching, resulting if type for value to be i32
        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // let mut iter = list.into_iter();

        let mut expected = 3;
        for el in list {
            assert_eq!(el, expected);

            expected -= 1;
        }
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        {
            let mut iter = list.iter();
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&1));
        }

        // Collection is not moved, nor altered
        assert_eq!(list.peek(), Some(&3));
    }
}
