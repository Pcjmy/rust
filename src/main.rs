fn main() {
    let myarray: [u32; 5] = [1, 2, 3, 4, 5];

    println!("myarray[1] = {}", myarray[1]);

    // let index = "5".parse::<usize>().unwrap();

    // println!("myarray[5] = {}", myarray[index]);

    let mut mybuffer: [u32; 32 * 1024] = [0; 32 * 1024];
    println!("mybuffer[1024] = {}", mybuffer[1024]);

    mybuffer[1024] = 13;

    println!("mybuffer[1024] = {}", mybuffer[1024]);
}
