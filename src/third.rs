use std::{ops::Deref, rc::Rc};

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> Self {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                // This will increase a reference count of that node
                next: self.head.clone(),
            })),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem )
    }

    pub fn tail(&self) -> Self {
        // and_then is the same as map, but it does not wrap result into Option,
        // This is delegated to closure
        // and_then returns f(x), where f: x -> Some(y)
        let b = self.head.as_ref().and_then(|e| 
            /* e.next is option itself, so no need to wrap it again */ 

            // need to state .clone(), because Rc does not implement Copy, so move will occur.
            // Implement Copy if you want your type to be copied instead of moving (like i32)
            // Implement Clone if you want to give 'copy' capabilities on demand,
            // having still default move semantics
            e.next.clone());

        // map returns Some(f(x)), where f: x -> y
        let c = self.head.as_ref().map(
            |e| {
                e.next.clone()
                // no matter what you will do here, map will wrap it up into Option.
                // In particular Option::None will become Some(None)
                   
                // We need to return what's inside e.next (which is an Option itself)
            });
                
        // use flatten to reduce nesting of Option, so Some(None) becomes None, Some(Some(x)) -> Some(x)
        let c = c.flatten();

        List { head: c }
    }   
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            // try_unwrap will return Ok(content) if this is last owner of that data
            if let Ok(mut node) = Rc::try_unwrap(node) {
                // set cursor to next element on the list
                head = node.next.take();

                // node goes out of scope, will call Drop
                // there is node.next = None (because we called .take())
                // so no recursive stack abuse threat here
            } else {
                // There is another list somewhere, which has this node as an element
                // Don't delete it yet
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);

    }

    #[test]
    fn break_the_stack() {
        {
            let mut list = List::new();

            for i in 1..100000 {
                list = list.prepend(i);
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

}

