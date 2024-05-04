fn main() {
    let a = 10;
    let b = 'A';

    let mytuple = (a, b);

    let (c, d) = mytuple;
    println!("c={} d={}", c, d);

    println!("mytuple.0 = {}", mytuple.0);
    println!("mytuple.1 = {}", mytuple.1);
}
