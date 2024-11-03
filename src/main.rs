enum Symbol {
    Char(char),
    Number,
}

fn main() {
    let letter = Symbol::Char('A');

    if let Symbol::Char(x) = letter {
        println!("{}", x);
    }
}
