#[derive(Debug)]
struct BuahBuahan {
    apel: String,
    melon: String,
    harga: u32,
}

impl BuahBuahan {
    fn new(apel: String, melon: String, harga: u32) -> BuahBuahan {
        BuahBuahan { apel, melon, harga }
    }
}
fn main() {
    let buah = BuahBuahan::new(String::from("apel"), String::from("melon"), 2500);

    println!("{:#?}", buah);
}
