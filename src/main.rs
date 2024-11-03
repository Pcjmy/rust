fn fibonacci(n: u64) -> u64 {
    if n < 2 {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn main() {
    println!("{:?}", fibonacci(10));
}
