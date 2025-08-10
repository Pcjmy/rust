// ConsList 每一项包含两个元素：当前项和下一项

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Cons(0, Box::new(List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))));
}
