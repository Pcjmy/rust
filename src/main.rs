fn main() {
    // let mut v: Vec<i32> = Vec::new();
    // for i in 0..10 {
    //     v.push(i);
    // }

    let mut v: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{:?}", v.pop());
    println!("{:?}", v.len());
    println!("{:?}", v.capacity());
    v.push(9);
    v.push(10);
    println!("{:?}", v.len());
    println!("{:?}", v.capacity());
}
