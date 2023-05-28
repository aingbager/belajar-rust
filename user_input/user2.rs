use std::io;

fn main() {
    println!("masukan inputan: ");
    let mut hasil = String::new();
    io::stdin()
        .read_line(&mut hasil)
        .expect("gagal membaca input");

    println!("hasil inputan: {}", hasil);
}
