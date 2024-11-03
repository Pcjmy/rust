fn calc(method: fn(u32, u32) -> u32, a: u32, b: u32) -> u32 {
    method(a, b)
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn sub(a: u32, b: u32) -> u32 {
    a - b
}

fn main() {
    println!("{}", calc(add, 10, 20));
    println!("{}", calc(sub, 20, 10));
}
