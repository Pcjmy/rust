use std::thread;

fn main() {
    // move: 将环境中的值移到闭包内部
    // 使用场景-多线程：从主线程移动值到子线程

    let hello_message = "Hello World!";

    thread::spawn(move || {
        println!("{}", hello_message);
    }).join();
}
