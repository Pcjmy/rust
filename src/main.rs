fn main() {
    let mut sum = 0;
    let mut n = 1;
    loop {
        sum += n;
        n += 1;
        if n > 100 {
            break;
        }
    }

    println!("1 + 2 + ... + 100 = {}", sum);
}
