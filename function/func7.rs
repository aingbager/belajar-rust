use std::io;

fn kali(a: i32, b: i32) -> i32 {
    let hasil = a * b;
    return hasil;
}

fn main() {
    let mut input1 = String::new();
    println!("masukan angka:");
    io::stdin().read_line(&mut input1).expect("error");

    let a: i32 = input1.trim().parse().expect("error");
    let b: i32 = 5;
    let hasil = kali(a, b);

    println!("hasilnya: {}", hasil);
}
