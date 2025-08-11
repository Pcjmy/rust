fn main() {
    let mut v: Vec<i32> = Vec::new();
    for i in 0..10 {
        v.push(i);
    }
    println!("{:?}", v[9]);
}
