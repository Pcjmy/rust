fn foo() -> ! {
    panic!("This call never returns.");
}

fn main() {
    let a = if true {
        10
    } else {
        foo()
    };
    println!("{}", a);
}
