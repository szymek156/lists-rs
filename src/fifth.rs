pub struct List<'a, T> {
    head: Link<T>,
    tail: Option<&'a mut Node<T>>, // NEW!
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<'a, T> List<'a, T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }


    // error[E0499]: cannot borrow `list` as mutable more than once at a time
    // --> src/fifth.rs:75:9
    // |
    // 74 |         list.push(1);
    // |         ---- first mutable borrow occurs here
    // 75 |         list.push(2);
    // |         ^^^^
    // |         |
    // |         second mutable borrow occurs here
    // |         first borrow later used here
    //
    // Error is caused by lifetime parameter, without it: &mut self it compiles
    // We define a list like:
    // tail: Option<&mut Node<T>>,
    // This reference needs to have specified lifetime parameter, so add it:
    // In impl declaration, but not in push method:
    // tail: Option<&'a mut Node<T>>,
    // impl<'a, T> List<'a, T> {
    // pub fn push(& mut self, elem: T)  
    // Then:
    // Compiler is unable to infer lifetime for this line:
    // old_tail.next.as_deref_mut()
    // Because:
    // First: it cannot outlive anonymous lifetime for &mut self
    // Second: but lifetime must be valid for 'a, specified in impl<'a, T>
    // There are two lifetimes, not connected to each other, anonymous &mut self, and 'a
    // In order to tell the compiler, that 'a must live as long as &mut self, add lifetime parameter:
    // &'a mut self
    // That connects two lifetimes

    // My understanding what happens next, not sure if correct:
    // But now we require that mutable reference:
    // old_tail.next.as_deref_mut() lives as long as self
    // and upon second call to push
    // we create another mutable reference with such lifetime, but previous still exists (will exist as long as &self exists)
    // so we have two mutable references with lifetime of &self - that's where compiler explodes, you cannot have two
    pub fn push<'b>(&'b mut self, elem: T) {
        let new_tail = Box::new(Node {
            elem: elem,
            // When you push onto the tail, your next is always None
            next: None,
        });

        // Put the box in the right place, and then grab a reference to its Node
        let new_tail: Option<&'a mut Node<T>> = match self.tail.take() {
            Some(old_tail) => {
                // If the old tail existed, update it to point to the new tail
                old_tail.next = Some(new_tail);
                old_tail.next.as_deref_mut()
            }
            None => {
                // Otherwise, update the head to point to it
                self.head = Some(new_tail);
                self.head.as_deref_mut()
            }
        };

        self.tail = new_tail;
    }

    // pub fn pop(&'a mut self) -> Option<T> {
    //     // Grab the list's current head
    //     self.head.take().map(|head| {
    //         let head = *head;
    //         self.head = head.next;

    //         // If we're out of `head`, make sure to set the tail to `None`.
    //         if self.head.is_none() {
    //             self.tail = None;
    //         }

    //         head.elem
    //     })
    // }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        // assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        // list.push(3);

        // // Check normal removal
        // assert_eq!(list.pop(), Some(1));
        // assert_eq!(list.pop(), Some(2));

        // // Push some more just to make sure nothing's corrupted
        // list.push(4);
        // list.push(5);

        // // Check normal removal
        // assert_eq!(list.pop(), Some(3));
        // assert_eq!(list.pop(), Some(4));

        // // Check exhaustion
        // assert_eq!(list.pop(), Some(5));
        // assert_eq!(list.pop(), None);
    }
}
