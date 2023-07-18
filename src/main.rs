use linked_list_exercise::List;

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let list = List::from(v);
    println!("{:?}", list.into_vec());

    let mut n = List::new();
    n.push(1);
    n.push(2);
    n.push(3);
    n.push(4);
    println!("{:?}", &n);
    n.insert(2, 7);
    println!("{:?}", &n);
    n.remove(2);
    println!("{:?}", n);
}

#[cfg(test)]
mod test {
    use linked_list_exercise::List;
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