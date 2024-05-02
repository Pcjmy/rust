// 创建变量：let 关键字
// 变量默认是不可变的
// 可变变量：变量名称前加 mut

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);
}
