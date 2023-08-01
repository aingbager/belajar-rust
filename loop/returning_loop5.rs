fn main() {
    //belajar returning loop
    let mut angka = 0;

    let hasil = loop {
        angka += 1;
        if angka == 10 {
            break angka * 3;
        }
    };

    println!("angka: {hasil}");
}
