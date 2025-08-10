fn bar() -> Result<u32, &'static str> {
    Ok(0)
}

fn foo() -> Result<i32, &'static str> {
    // match bar() {
    //     Ok(a) => Ok(a as i32),
    //     Err(e) => Err(e),
    // }

    let a = bar()?;
    Ok(a as i32)
}

fn main() {
    println!("{:?}", foo());
}
