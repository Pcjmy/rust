fn main() {
    let mut myarray = [1, 2, 3];

    for i in myarray.iter_mut() {
        *i *= 2;
    }

    for i in myarray.iter() {
        println!("{}", i);
    }
}
