fn echo(s: &str) {
    println!("{:?}", s);
}

struct Foo {
    name: String,
}

fn main() {
    // "Hello World!" 字符串字面量存储在程序的数据段中
    // "Hello World!" 叫做字符串的字面量

    // str类型是底层的字符串切片类型，通常不会直接使用
    // 在实际编程中，我们几乎总是使用 &str 字符串引用
    // str 表示存储在内存中的实际字符串数据序列
    // &str 可以引用数据段中的内容，也可以是堆里面的内容
    let s: &'static str = "Hello World!";

    let mut t = String::from(s);
    // String 类型是可变的字符串类型，拥有所有权
    // 可以动态增长和修改其内容
    // 数据存储在堆内存中，具有更灵活的内存管理能力
    t.push_str("!!");

    println!("{:?}", t);

    echo(s);
    echo(&t);

    let foo = Foo { name: String::from(s) };
    println!("{:?}", foo.name);
}
