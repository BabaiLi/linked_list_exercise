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