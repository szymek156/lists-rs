use std::mem;

#[derive(PartialEq, Debug)]
pub struct List {
    head: Link,
}
#[derive(PartialEq, Debug)]
struct Node {
    elem: i32,
    next: Link,
}
#[derive(PartialEq, Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        // head -> Node
        // new_node -> Node, head -> Empty
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
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
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let poped_value = mem::replace(&mut self.head, Link::Empty);

        match poped_value {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;

                return Some(node.elem);
            }
        }
    }

    pub fn push_back(&mut self, elem: i32) {
        let link = Link::More(Box::new(Node {
            elem: elem,
            next: Link::Empty,
        }));

        let mut tail = &mut self.head;

        loop {
            match tail {
                Link::Empty => break,
                Link::More(more) => tail = &mut more.next,
            }
        }

        *tail = link;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_back_works() {
        let mut list = List::new();

        assert_eq!(list, List { head: Link::Empty });

        list.push_back(69);

        assert_eq!(
            list,
            List {
                head: Link::More(Box::new(Node {
                    elem: 69,
                    next: Link::Empty
                }))
            }
        );

        list.push_back(13);

        assert_eq!(
            list,
            List {
                head: Link::More(Box::new(Node {
                    elem: 69,
                    next: Link::More(Box::new(Node {
                        elem: 13,
                        next: Link::Empty
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
}
