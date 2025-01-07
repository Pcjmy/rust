#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
}

fn main() {
    let p = Person { name: "jack" };
    println!("{:?}", p);
}
