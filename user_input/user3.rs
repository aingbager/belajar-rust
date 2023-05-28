use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Masukkan sesuatu: ");
    io::stdout().flush().expect("Gagal mengeluarkan output"); // Memastikan output pertama ditampilkan

    io::stdin()
        .read_line(&mut input)
        .expect("Gagal membaca input");

    print!("Anda memasukkan: {}", input);
}
