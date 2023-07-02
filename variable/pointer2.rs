fn main() {
    let angka: i32 = 555;

    let angka2: &i32 = &angka;

    println!("angka pointer: {}", *angka2);
    println!("alamaat memory angka2: {:p}", angka2);
    println!("alamaat memory angka: {:p}", &angka);
}
