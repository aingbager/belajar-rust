enum Buah {
    Anggur,
    Melon,
}
fn main() {
    let makanan: Buah = Buah::Anggur;

    match makanan {
        Buah::Anggur => {
            println!("makanan enak");
        }
        Buah::Melon => {
            println!("makanan faforit");
        }
    }
}
