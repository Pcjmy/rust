// 创建变量：let 关键字
// 变量默认是不可变的
// 可变变量：变量名称前加 mut
// 常量：const 关键字
// Shadowing: 隐藏

fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{} {} {}", c, z, heart_eyed_cat);
}
