/// Singly linked list

pub struct SLLList<T> {
    pub head: Box<SLLNode<T>>,
}
impl<T> SLLList<T> {
    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        let mut current = &mut self.head;
        while let Some(next) = current.next {
            current = &mut next;
        }
        None
    }
}

impl<T> SLLList<T> {
    pub fn new(element: T) -> Self {
        Self {
            head: SLLNode::new(element, None),
        }
    }
}

#[derive(Debug)]
pub struct SLLNode<T> {
    data: T,
    next: Option<Box<SLLNode<T>>>,
}

impl<T> SLLNode<T> {
    pub fn new(data: T, next: Option<Box<SLLNode<T>>>) -> Box<Self> {
        Box::new(Self {
            data,
            next
        })
    }

    pub fn insert_after(&mut self, mut node: Box<SLLNode<T>>) {
        let self_next = self.next.take();
        node.next = self_next;
        self.next = Some(node);
    }
}

pub struct SLLNodeIter<'a, T> {
    current: Option<&'a Box<SLLNode<T>>>,
}

impl<'a, T> Iterator for SLLNodeIter<'a, T> {
    type Item = &'a Box<SLLNode<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current = self.next();
        self.current
    }
}

#[cfg(test)]
mod test {
    use crate::linkedlist::sll::{SLLList, SLLNode};

    #[test]
    pub fn test_basic_list() {
        let mut list = SLLList::new(0);
        for i in 1..=10 {
            list.head.insert_after(SLLNode::new(i, None))
        }
        let mut active_node = list.head;
        loop {
            println!("{}", active_node.data);
            if active_node.next.is_none() {
                break;
            } else {
                active_node = active_node.next.unwrap();
            }
        }
    }
}