use std::time::{SystemTime};

fn main() {
    // SystemTime 是系统时间
    // 通过系统调用请求操作系统返回的系统时间
    let now = SystemTime::now();
    println!("{:?}", now);

    let timestamp = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    println!("timestamp = {:?}", timestamp);
}
