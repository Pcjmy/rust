fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("{}", largest(10, 20));
    println!("{}", largest(10.0, 20.0));
    let a: f64 = 3.14;
    let b: f64 = 2.71;
    println!("{}", largest(a, b));
}
