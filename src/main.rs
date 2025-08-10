// ConsList 每一项包含两个元素：当前项和下一项

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::read("hello.txt")?;
    Ok(())
}
