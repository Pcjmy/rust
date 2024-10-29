fn main() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &arr[0..3];
    println!("slice[0]={}, len={}", slice[0], slice.len());

    let slice2 = &arr[3..5];
    println!("slice2[0]={}, len={}", slice2[0], slice2.len());

    let mut slice3 = &mut arr[..];
    slice3[0] = 6;
    println!("arr[0]={}", arr[0]);
}
