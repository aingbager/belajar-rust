fn main() {
    let mut i = 1;

    let 'NAMA: angka = for i in 1..15 {
        i += 1;

        if i > 10 {
            break 'NAMA;
        }
    };
    println!("{}", angka);
}
