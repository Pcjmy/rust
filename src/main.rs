fn main() {
    // let mut v: Vec<i32> = Vec::new();
    // for i in 0..10 {
    //     v.push(i);
    // }

    let mut v: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    // for i in 0..v.len() {
    //     println!("v[{:?}]={:?}", i, v[i]);
    // }

    // for e in v.iter() {
    //     println!("{:?}", e);
    // }

    for e in v.iter_mut() {
        *e *= 2;
    }

    for i in 0..v.len() {
        println!("v[{:?}]={:?}", i, v[i]);
    }
}
