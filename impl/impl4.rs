#[derive(Debug)]
struct Baju {
    warna: String,
    warna2: String,
}

fn main() {
    let user1 = Baju {
        warna: String::from("merah"),
        warna2: String::from("kuning"),
    };

    println!("pendefinisian sruct");
    println!("{:?}", user1);
}
