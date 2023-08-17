enum Sayuran {
    Bayam,
    Tomat,
    Cabe,
}

fn main() {
    let makan: Sayuran = Sayuran::Tomat;

    match makan {
        Sayuran::Bayam => {
            println!("banyak gizinya");
        }
        Sayuran::Tomat => {
            println!("banyak vitaminnya");
        }
        Sayuran::Cabe => {
            println!("pedas cok");
        }
    }
}
