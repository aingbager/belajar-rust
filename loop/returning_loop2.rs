fn main() {
    //returning loop
    let mut angka = 0;

    let hasil = loop {
        angka += 1;

        if angka == 100 {
            break angka / 2;
        }
    };

    println!("hasilnya: {}", hasil);
}
