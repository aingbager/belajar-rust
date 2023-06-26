enum Food {
    Bakso,
    MieAyam,
    Martabak,
}

fn main() {
    let favorit: Food = Food::Bakso;

    //match untuk seleksi
    match favorit {
        Food::Bakso => {
            println!("best makanan faporite");
        }
        Food::MieAyam => {
            println!("wow mie ayam");
        }
        Food::Martabak => {
            println!("makanan enak");
        }
    }
}
