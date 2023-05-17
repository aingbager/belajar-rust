fn main() {
    let mut number = [1, 2, 3, 4, 5];
    println!("{:?}", number);

    let data0 = number[0];
    println!("index array ke 0: {data0}");
    let data1 = number[1];
    println!("index array ke 1: {data1}");

    number[1] = 100;
    number[3] = 400;

    println!(":number?");
}
