enum Angka {
    Angka1,
    Angka2,
}

fn main() {
    let number: Angka = Angka::Angka1;

    match number {
        Angka::Angka1 => {
            println!("ini angka 1");
        }
        Angka::Angka2 => {
            println!("ini angka 2");
        }
    }
}
