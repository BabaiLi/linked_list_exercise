pub use std::rc::Rc;

pub struct RcList<T> {
    pub head: RcListPtr<T>,
}

pub type RcListPtr<T> = Option<Rc<RcNode<T>>>;

pub struct RcNode<T> {
    pub val: T,
    pub next: RcListPtr<T>,
}

impl<T> RcList<T> {
    pub fn new() -> Self {
        RcList { head: None }
    }
    pub fn prepend(&mut self, elem: T) -> RcList<T> {
        RcList { head: Some(Rc::new(RcNode {
            val: elem,
            next: self.head.clone(),
        }))}
    }
    pub fn tail(&mut self) -> RcList<T> {
        RcList { head: self.head.as_ref().and_then(|node| node.next.clone())}
    }
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val )
    }
}

impl <T> Drop for RcList<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct IntoIter<T>(RcList<T>);
impl<T> RcList<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.head.take().and_then(|rc_node| {
            if let Ok(mut node) = Rc::try_unwrap(rc_node) {
                self.0.head = node.next.take();
                Some(node.val)
            } else {
                None
            }
        })
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a RcNode<T>>,
}
impl <T> RcList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
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
    next: Option<&'a mut RcNode<T>>
}
impl<T> RcList<T>{
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        let mut next = None;
        if let Some(ref mut rc_node) = self.head {
            next = Rc::get_mut(rc_node);
        }
        IterMut { next }
    }
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().and_then(|node| {
            if let Some(ref mut rc_node) = node.next {
                self.next = Rc::get_mut(rc_node);
            }
            Some(&mut node.val)
        })
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for RcList<T> {
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
    use super::RcList;

    #[test]
    fn basics() {
        let mut list = RcList::new();
        assert_eq!(list.head(), None);

        list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        list = list.tail();
        assert_eq!(list.head(), Some(&2));

        list = list.tail();
        assert_eq!(list.head(), Some(&1));

        list = list.tail();
        assert_eq!(list.head(), None);

        let list = list.tail();
        assert_eq!(list.head(), None);
    }
    #[test]
    fn iter() {
        let list = RcList::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter_mut() {
        let mut list = RcList::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter_mut();
        if let Some(node) = iter.next() {
            *node = 6;
        }
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
        println!("now list: {:?}", list);
    }
    #[test]
    fn into_iter() {
        let list = RcList::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn format() {
        let mut list: RcList<_> = RcList::new();
        assert_eq!(format!("{:?}", &list), "None");

        list = list.prepend(1).prepend(2).prepend(3).prepend(4);
        assert_eq!(format!("{:?}", &list), "4 -> 3 -> 2 -> 1 -> None");

    }
}