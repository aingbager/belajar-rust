fn main() {
    let mut angka = 0;

    loop {
        println!("angka ke: {angka}");
        angka += 1;

        if angka == 50000 {
            break;
        }
    }
    println!("program berakhir");
}
