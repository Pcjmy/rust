fn change(s: &mut String) {
    s.push_str(" changed!");
}

fn main() {
    let mut s = String::from("Hello World!");
    change(&mut s);
    println!("{}", s);
}
