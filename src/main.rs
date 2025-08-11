use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // 一个值可以有多个所有者
    let four = Rc::new(List::Cons(4, Rc::new(List::Nil)));
    let zero_one = List::Cons(0, Rc::new(List::Cons(1, four.clone())));
    let two_three = List::Cons(2, Rc::new(List::Cons(3, four)));
}
