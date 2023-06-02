struct MyNumber {
    nama: String,
    usia: u16,
    alamat: String,
}

fn main() {
    let udin = MyNumber {
        nama: String::from("udin"),
        usia: 49,
        alamat: String::from("kp.ciantra rt07/04 ds.ciantra"),
    },

    println!("nama: {}", udin.nama);
    println!("usia: {}", udin.usia);
    println!("alamat: {}", udin.alamat);
}
