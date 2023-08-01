fn kali(a: i32, b: i32) -> i32 {
    let hasil = a * b;
    return hasil;
}

fn hasil(a: i32, b: i32) {
    let hasil = kali(a, b);
    println!("hasilnya {}", hasil);
}

fn main() {
    //belajar function
    let a = 5;
    let b = 3;

    hasil(a, b);
}
