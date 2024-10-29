// 元组结构
struct Pair(i32, f32);

// 标准的C结构
struct Person {
    name: String,
    age: u32,
}

// 单元结构（无字段，通常在泛型里使用较多）
struct Unit;

fn main() {
    let pair = Pair(10, 7.3);
    println!("{}", pair.0);

    let jack = Person {
        name: String::from("jack"),
        age: 6,
    };
    println!("name={} age={}", jack.name, jack.age);

    let unit = Unit;
}
