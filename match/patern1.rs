
fn main() {
    //pattern | dan .. atau ..=

    let angka = 6;

    match angka{
        1 | 2 => println!("satu atau dua"),
        3..=5 => println!("tiga sampai lima"),
        6 => println!("ini enam"),
        _ => println!("tidak diketahui"),
    }
}
