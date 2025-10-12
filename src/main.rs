use std::rc::Rc;
use std::cell::RefCell;

struct Node<T> {
    value: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

fn main() {
    let mut root = Node::<u32>{value: 1, left: None, right: None};
    let left = Node::<u32>{value: 2, left: None, right: None};
    let right = Node::<u32>{value: 3, left: None, right: None};
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));

    // println!("root = {:?}", root.value);
    // println!("left = {:?}", root.left.unwrap().value);
    // println!("right = {:?}", root.right.unwrap().value);

    if let Some(ref mut x) = root.left {
        x.borrow_mut().value = 4;
    }

    if let Some(ref x) = root.left {
        println!("left = {:?}", x.borrow().value);
    }
}
