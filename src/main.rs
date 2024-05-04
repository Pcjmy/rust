fn main() {
    let myarray: [u32; 5] = [1, 2, 3, 4, 5];

    println!("myarray[1] = {}", myarray[1]);

    let index = "5".parse::<usize>().unwrap();

    println!("myarray[5] = {}", myarray[index]);
}
