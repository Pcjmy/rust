use std::time::{SystemTime, Duration};
use std::thread::sleep;

fn main() {
    // SystemTime 是系统时间
    // 通过系统调用请求操作系统返回的系统时间
    let now = SystemTime::now();
    println!("{:?}", now);

    let timestamp = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    println!("timestamp = {:?}", timestamp);

    sleep(Duration::from_secs(3));

    println!("elapsed = {:?}", now.elapsed().unwrap());
}
