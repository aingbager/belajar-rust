fn main() {
    //belajar match

    let angka = 4;

    match angka {
        1 | 2 => println!("angka 1 atau 2"),
        3..=6 => println!("angka 3 sampai samagdengan 6"),
        _ => println!("tidak diketahui"),
    }
}
