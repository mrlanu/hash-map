type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Link<T>,
    size: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn push(&mut self, el: T) {
        let mut new_node = Box::new(Node::new(el));
        new_node.next = self.head.take();
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            if self.size > 0 {
                self.size -= 1;
            }
            node.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.element)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

pub struct IntoIter<T>(LinkedList<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.element
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[derive(Debug)]
pub struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(el: T) -> Self {
        Self {
            element: el,
            next: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn basic() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.size, 0);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.size(), 3);

        let mut el = list.pop();

        assert_eq!(el, Some(3));
        assert_eq!(list.size(), 2);

        list.pop();
        list.pop();
        el = list.pop();

        assert_eq!(el, None);
        assert_eq!(list.size(), 0);

        list.push(66);
        let mut new_value = list.peek_mut().unwrap();
        *new_value = 55;

        assert_eq!(list.pop(), Some(55));
    }
}
