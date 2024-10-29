// 标准的C结构
#[derive(Debug)] // 派生属性
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let jack = Person {
        name: String::from("jack"),
        age: 6,
    };
    println!("{:?}", jack);
}
