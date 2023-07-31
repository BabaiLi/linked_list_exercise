pub mod list;
pub mod rc_list;
pub use list::List;
pub use rc_list::RcList;

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let list = List::from(v);
    println!("{:?}", list.into_vec());

    let mut n = List::new();
    n.push(1);
    n.push(2);
    n.push(3);
    n.push(4);
    println!("List: {:?}", &n);
    n.insert(2, 7);
    println!("List: {:?}", &n);
    n.remove(2);
    println!("List: {:?}", n);

    let mut rc_list = RcList::new().prepend(1).prepend(2);
    println!("RcList: {:?}", &rc_list);
    println!("Head: {:?}", &rc_list.head());
    println!("Tail: {:?}", rc_list.tail());
}