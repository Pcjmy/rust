fn main() {
    let a: Result<u32, &'static str> = Result::Ok(1);
    println!("{:?}", a);

    let b: Result<u32, &'static str> = Result::Err("result error");
    println!("{:?}", b);
}
