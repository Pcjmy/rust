type Method = fn(u32, u32) -> u32;

fn calc(method: &str) -> Method {
    match method {
        "add" => add,
        "sub" => sub,
        _ => unimplemented!(),
    }
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn sub(a: u32, b: u32) -> u32 {
    a - b
}

fn main() {
    println!("{}", calc("add")(10, 20));
    println!("{}", calc("sub")(20, 10));
}
