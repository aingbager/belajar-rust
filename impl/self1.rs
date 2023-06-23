#[derive(Debug)]
struct Orang {
    nama1: String,
    nama2: String,
}

impl Orang {
    fn user(nama1: String, nama2: String) -> Self {
        Self { nama1, nama2 }
    }
}

fn main() {
    let human1 = Orang {
        nama1: String::from("udin"),
        nama2: String::from("surudin"),
    };

    println!("human1: {:#?}", human1);

    let human2 = Orang::user(String::from("otong"), String::from("surotong"));
    println!("human2: {:#?}", human2);
}
