pub struct List<T> {
    left: Stack<T>,
    right: Stack<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            left: Stack::new(),
            right: Stack::new(),
        }
    }

    pub fn push_left(&mut self, elem: T) {
        self.left.push(elem)
    }

    pub fn push_right(&mut self, elem: T) {
        self.right.push(elem)
    }

    pub fn pop_left(&mut self) -> Option<T> {
        self.left.pop()
    }

    pub fn pop_right(&mut self) -> Option<T> {
        self.right.pop()
    }

    pub fn peek_left(&self) -> Option<&T> {
        self.left.peek()
    }

    pub fn peek_right(&self) -> Option<&T> {
        self.right.peek()
    }

    pub fn peek_left_mut(&mut self) -> Option<&mut T> {
        self.left.peek_mut()
    }

    pub fn peek_right_mut(&mut self) -> Option<&mut T> {
        self.right.peek_mut()
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn walk_aboot() {
        let mut list = List::new();

        list.push_left(0);
        list.push_right(1);
        assert_eq!(list.peek_left(), Some(&0));
        assert_eq!(list.peek_right(), Some(&1));

        list.push_left(2);
        list.push_left(3);
        list.push_right(4);
    }
}

pub struct Stack<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}
