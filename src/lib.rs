pub mod rc_list;
pub use rc_list::RcList;

pub struct List<T> {
    pub head: ListPtr<T>,
}

pub type ListPtr<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub val: T,
    pub next: ListPtr<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            val: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.head
            .as_ref()
            .map(|node| {
                &node.val
            })
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head
            .as_mut()
            .map(|node| {
                &mut node.val
            })
    }
    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut curr = self.head.take();

        while let Some(mut node) = curr.take() {
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            curr = next;
        }
        self.head = prev.take();
    }
    pub fn len(&self) -> usize {
        let mut n = 0;
        let mut list = self.head.as_ref();
        while let Some(node) = list.take() {
            list = node.next.as_ref().take();
            n += 1;
        }
        n
    }
    pub fn insert(&mut self, pos: usize, elem: T) {
        let mut pos_ = self.len() - (pos + 1);
        let mut curr = &mut self.head;

        while pos_ > 0 {
            curr = match curr.as_mut() {
                Some(node) => &mut node.next,
                None => panic!("Error"),
            };
            pos_ -= 1;
        }

        match curr.take() {
            Some(mut node) => {
                let new_node = Box::new(Node {
                    val: elem,
                    next: node.next,
                });
                node.next = Some(new_node);
                *curr = Some(node);
            },
            None => panic!("Error"),
        }
    }
    pub fn remove(&mut self, pos: usize) {
        let mut pos = self.len() - (pos + 1);
        let mut curr = &mut self.head;

        while pos > 0 {
            curr = &mut curr.as_mut().unwrap().next;
            pos -= 1;
        }

        let node = curr.take();
        *curr = node.unwrap().next;
    }
    pub fn into_vec(self) -> Vec<T> {
        let mut v: Vec<_> = self.into_iter().collect();
        v.reverse();
        v
    }
}

pub struct IntoIter<T>(List<T>);
impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}
impl <T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { next: self.head.as_deref() }
    }
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.val
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}
impl<T> List<T>{
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.val
        })
    }
}

impl<T> From<Vec<T>> for List<T> {
    fn from(v: Vec<T>) -> Self {
        let mut head = List::new();
        for i in v.into_iter() {
            head.push(i);
        }
        head
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(node) = cur_link {
            cur_link = node.next;
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for node in self.iter() {
            write!(f, "{:?} -> ", node)?;
        }
        write!(f, "None")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| {
            *value = 42
        });

    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn len() {
        let list = List::from(vec![1, 2, 3, 4]);
        let empty: List<i32> = List::new();

        assert_eq!(list.len(), 4);
        assert_eq!(empty.len(), 0);
    }

    #[test]
    fn format() {
        let list = List::from(vec![1, 2, 3, 4]);
        assert_eq!(format!("{:?}", list), "4 -> 3 -> 2 -> 1 -> None");
    }

    #[test]
    fn insert_and_remove() {
        let mut list = List::from(vec![1, 2, 3]);
        list.insert(1, 10);
        assert_eq!(format!("{:?}", list), "3 -> 2 -> 10 -> 1 -> None");
        list.insert(3, 11);
        assert_eq!(format!("{:?}", list), "3 -> 11 -> 2 -> 10 -> 1 -> None");

        list.remove(1);
        assert_eq!(format!("{:?}", list), "3 -> 11 -> 2 -> 1 -> None");
        list.remove(2);
        assert_eq!(format!("{:?}", list), "3 -> 2 -> 1 -> None");
    }
}