fn main() {
    //type data slice
    //
    //type data slice
    let number = [45, 2, 34, 6, 5];
    println!("size number: {}", number.len());
    println!("index ke 0: {}", number[0]);
    println!("index ke 1: {}", number[1]);

    println!("\n==================\n");

    let number_slice = &number[0..3];
    println!("{:?} ,size: {}", number_slice, number_slice.len());
}
