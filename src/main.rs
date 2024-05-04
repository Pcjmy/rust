// 创建变量：let 关键字
// 变量默认是不可变的
// 可变变量：变量名称前加 mut
// 常量：const 关键字
// Shadowing: 隐藏

// cargo build --release
fn main() {
    let a: u32 = "4294967295".parse::<u32>().unwrap();
    let b: u32 = 1;

    let sum = a + b;
    println!("sum = {:?}", sum);
}
