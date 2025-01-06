fn echo(s: &String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("Hello World!");
    echo(&s);
    println!("{}", s);
}
