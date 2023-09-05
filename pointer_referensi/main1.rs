fn main() {
    //belajar pinter dan reperensi
    let angka1 = 5;
    println!("angka1: {}", angka1);

    let angka2 = &angka1; //untuk mengambil memory angka1
    println!("pointer angka1: {:p}", angka2);

    let angka3 = *angka2;
    println!("{}", angka3);
}
