
// std::fs::read

fn main() {
    let r = std::fs::read("hello.txt");
    match r {
        Ok(data) => {
            println!("{:?}", std::str::from_utf8(&data).unwrap());
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
