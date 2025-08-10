// ConsList 每一项包含两个元素：当前项和下一项

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let a = [0; 1024 * 512];
    let a_box = Box::new(a);
}
