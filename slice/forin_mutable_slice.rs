fn main() {
    //belajar for in mutable slice
    let mut angka = [1,2,3,4];
    println!("angka before: {:?}",angka);

    let angka_slice = &mut angka[..];

    for result in &mut angka_slice[..]{
        *result += 1;
    }

    println!("angka after: {:?}",angka);
}
