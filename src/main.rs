#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    // 构造方法
    fn new(x: u64, y: u64) -> Point {
        Point { x, y }
    }
}

fn main() {
    let p = Point::new(10, 20);
    println!("{:?}", p);
}
