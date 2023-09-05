#[derive(Debug)]
enum Angka {
    Angka1,
    Angka2,
}
fn main() {
    //belajar enum
    let angka_favorite: Angka = Angka::Angka1;

    match angka_favorite {
        Angka::Angka1 => {
            println!("makanan favorite");
        }
        Angka::Angka2 => {
            println!("makanan bukan favorite");
        }
    }
}
