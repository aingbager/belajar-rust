#[derive(Debug)]
struct Warna {
    merah: String,
    kuning: String,
}

impl Warna {
    fn user(merah: String, kuning: String) -> Warna {
        Warna { merah, kuning }
    }
}

fn main() {
    let warna1 = Warna::user(String::from("merah"), String::from("kuning"));

    println!("{:?}", warna1);
}
