#[derive(Debug)]
struct Hewan {
    ayam: String,
    bebek: String,
}
fn main() {
    //belajar struct(structure)
    let hewan: Hewan = Hewan {
        ayam: String::from("makanan enak faforit"),
        bebek: String::from("makanan bukan faporit"),
    };

    println!("makanan faforit: {:?}", hewan);
}
