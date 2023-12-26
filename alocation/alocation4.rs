fn angka1() -> String {
    let mut nama = String::from("hello");
    {
        let nama2 = String::from("world");
        nama = nama2;
    }
}
fn main() {
    let angka = angka1();
    println!(":?", angka);
}
