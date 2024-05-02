// 创建变量：let 关键字
// 变量默认是不可变的
// 可变变量：变量名称前加 mut
// 常量：const 关键字
// Shadowing: 隐藏

fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is {}", x);
}
