fn main() {
    //slice

    let number: [i32; 4] = [2, 3, 55, 77];
    println!("number = {:?}", number);
    println!("size array = {:}", number.len());

    println!("\n=====================\n");
    println!("slice");

    let angka = &number[0..4];
    println!("size slice = {}", angka.len());
    println!("index ke: {}", angka[1]);
}
