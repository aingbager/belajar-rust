//fungsi dengan return
fn volume(panjang: i32, lebar: i32, tinggi: i32) -> i32 {
    let hasil = panjang * lebar * tinggi;
    return hasil;
}

//fungsi return tanpa mengunakan kata return
fn volume1(panjang: i32, lebar: i32, tinggi: i32) -> i32 {
    let hasil = panjang * lebar * tinggi;
    hasil //tanpa titik koma
}

//cara ketiga
fn volume2(panjang: i32, lebar: i32, tinggi: i32) -> i32 {
    panjang * lebar * tinggi //tanpa titik koma
}

fn main() {
    println!("{}", volume(2, 3, 4));
    let hasil = volume1(2, 3, 4);
    println!("{hasil}");

    let hasil2 = volume2(2, 3, 4);
    println!("{hasil2}");
}
